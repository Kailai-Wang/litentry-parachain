// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Inspired by:
//! - Moonbeam `pallet_parachain_staking`
//! implementations.

//! Auto-compounding functionality for staking rewards
#![allow(clippy::len_without_is_empty)]
use crate::{
	pallet::{
		AutoCompoundingDelegations as AutoCompoundingDelegationsStorage, BalanceOf, CandidateInfo,
		Config, DelegatorState, Error, Event, Pallet, Total,
	},
	types::{Bond, Delegator},
	weights::WeightInfo,
};
use codec::{Decode, Encode};
use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	ensure,
	traits::{Get, ReservableCurrency},
	RuntimeDebug,
};
use scale_info::TypeInfo;
use sp_runtime::{traits::Saturating, Percent};
use sp_std::{prelude::*, vec::Vec};

/// Represents the auto-compounding amount for a delegation.
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, PartialOrd, Ord)]
pub struct AutoCompoundConfig<AccountId> {
	pub delegator: AccountId,
	pub value: Percent,
}

/// Represents the auto-compounding [Delegations] for `T: Config`
#[derive(Clone, Eq, PartialEq, RuntimeDebug)]
pub struct AutoCompoundDelegations<T: frame_system::Config>(Vec<AutoCompoundConfig<T::AccountId>>);

impl<T> AutoCompoundDelegations<T>
where
	T: Config,
{
	/// Creates a new instance of [AutoCompoundingDelegations] from a vector of sorted_delegations.
	/// This is used for testing purposes only.
	#[cfg(test)]
	pub fn new(sorted_delegations: Vec<AutoCompoundConfig<T::AccountId>>) -> Self {
		Self(sorted_delegations)
	}

	/// Retrieves an instance of [AutoCompoundingDelegations] storage as [AutoCompoundDelegations].
	pub fn get_storage(candidate: &T::AccountId) -> Self {
		Self(<AutoCompoundingDelegationsStorage<T>>::get(candidate))
	}

	/// Inserts the current state to [AutoCompoundingDelegations] storage.
	pub fn set_storage(self, candidate: &T::AccountId) {
		<AutoCompoundingDelegationsStorage<T>>::insert(candidate, self.0)
	}

	/// Retrieves the auto-compounding value for a delegation. The `delegations_config` must be a
	/// sorted vector for binary_search to work.
	pub fn get_for_delegator(&self, delegator: &T::AccountId) -> Option<Percent> {
		match self.0.binary_search_by(|d| d.delegator.cmp(&delegator)) {
			Ok(index) => Some(self.0[index].value),
			Err(_) => None,
		}
	}

	/// Sets the auto-compounding value for a delegation. The `delegations_config` must be a sorted
	/// vector for binary_search to work.
	pub fn set_for_delegator(&mut self, delegator: T::AccountId, value: Percent) -> bool {
		match self.0.binary_search_by(|d| d.delegator.cmp(&delegator)) {
			Ok(index) =>
				if self.0[index].value == value {
					false
				} else {
					self.0[index].value = value;
					true
				},
			Err(index) => {
				self.0.insert(index, AutoCompoundConfig { delegator, value });
				true
			},
		}
	}

	/// Removes the auto-compounding value for a delegation.
	/// Returns `true` if the entry was removed, `false` otherwise. The `delegations_config` must be
	/// a sorted vector for binary_search to work.
	pub fn remove_for_delegator(&mut self, delegator: &T::AccountId) -> bool {
		match self.0.binary_search_by(|d| d.delegator.cmp(&delegator)) {
			Ok(index) => {
				self.0.remove(index);
				true
			},
			Err(_) => false,
		}
	}

	/// Returns the length of the inner vector.
	pub fn len(&self) -> u32 {
		self.0.len() as u32
	}

	/// Returns a reference to the inner vector.
	#[cfg(test)]
	pub fn inner(&self) -> &Vec<AutoCompoundConfig<T::AccountId>> {
		&self.0
	}

	/// Converts the [AutoCompoundDelegations] into the inner vector.
	#[cfg(test)]
	pub fn into_inner(self) -> Vec<AutoCompoundConfig<T::AccountId>> {
		self.0
	}

	// -- pallet functions --

	/// Delegates and sets the auto-compounding config. The function skips inserting auto-compound
	/// storage and validation, if the auto-compound value is 0%.
	pub(crate) fn delegate_with_auto_compound(
		candidate: T::AccountId,
		delegator: T::AccountId,
		amount: BalanceOf<T>,
		auto_compound: Percent,
	) -> DispatchResultWithPostInfo {
		// check that caller can reserve the amount before any changes to storage
		ensure!(
			<Pallet<T>>::get_delegator_stakable_free_balance(&delegator) >= amount,
			Error::<T>::InsufficientBalance
		);
		let delegator_state = if let Some(mut state) = <DelegatorState<T>>::get(&delegator) {
			// delegation after first
			ensure!(amount >= T::MinDelegation::get(), Error::<T>::DelegationBelowMin);
			ensure!(
				(state.delegations.0.len() as u32) < T::MaxDelegationsPerDelegator::get(),
				Error::<T>::ExceedMaxDelegationsPerDelegator
			);
			ensure!(
				state.add_delegation(Bond { owner: candidate.clone(), amount }),
				Error::<T>::AlreadyDelegatedCandidate
			);
			state
		} else {
			// first delegation
			ensure!(amount >= T::MinDelegatorStk::get(), Error::<T>::DelegatorBondBelowMin);
			ensure!(!<Pallet<T>>::is_candidate(&delegator), Error::<T>::CandidateExists);
			Delegator::new(delegator.clone(), candidate.clone(), amount)
		};
		let mut candidate_state =
			<CandidateInfo<T>>::get(&candidate).ok_or(Error::<T>::CandidateDNE)?;

		let auto_compounding_state = if !auto_compound.is_zero() {
			let auto_compounding_state = Self::get_storage(&candidate);
			Some(auto_compounding_state)
		} else {
			None
		};

		// add delegation to candidate
		let (delegator_position, less_total_staked) = candidate_state
			.add_delegation::<T>(&candidate, Bond { owner: delegator.clone(), amount })?;

		// lock delegator amount
		ensure!(
			<Pallet<T>>::get_delegator_stakable_free_balance(&delegator) >= amount,
			Error::<T>::InsufficientBalance
		);
		// Unless there is lock caused by other pallet/mechnism
		T::Currency::reserve(&delegator, amount)?;

		// adjust total locked,
		// only is_some if kicked the lowest bottom as a consequence of this new delegation
		let net_total_increase =
			if let Some(less) = less_total_staked { amount.saturating_sub(less) } else { amount };
		let new_total_locked = <Total<T>>::get().saturating_add(net_total_increase);

		// maybe set auto-compound config, state is Some if the percent is non-zero
		if let Some(mut state) = auto_compounding_state {
			state.set_for_delegator(delegator.clone(), auto_compound);
			state.set_storage(&candidate);
		}
		let delegator_delegation_count: u32 = delegator_state.delegations.0.len() as u32;
		let candidate_delegation_count: u32 = candidate_state.delegation_count as u32;
		<Total<T>>::put(new_total_locked);
		<CandidateInfo<T>>::insert(&candidate, candidate_state);
		<DelegatorState<T>>::insert(&delegator, delegator_state);
		<Pallet<T>>::deposit_event(Event::Delegation {
			delegator,
			locked_amount: amount,
			candidate,
			delegator_position,
			auto_compound,
		});
		let actual_weight = Some(T::WeightInfo::delegate_with_auto_compound(
			candidate_delegation_count,
			delegator_delegation_count,
			delegator_delegation_count,
		));
		Ok(actual_weight.into())
	}

	/// Sets the auto-compounding value for a delegation. The config is removed if value is zero.
	pub(crate) fn set_auto_compound(
		candidate: T::AccountId,
		delegator: T::AccountId,
		value: Percent,
	) -> DispatchResultWithPostInfo {
		let delegator_state =
			<DelegatorState<T>>::get(&delegator).ok_or(<Error<T>>::DelegatorDNE)?;
		ensure!(
			delegator_state.delegations.0.iter().any(|b| b.owner == candidate),
			<Error<T>>::DelegationDNE,
		);

		let mut auto_compounding_state = Self::get_storage(&candidate);
		let candidate_auto_compounding_count: u32 = auto_compounding_state.len() as u32;
		let state_updated = if value.is_zero() {
			auto_compounding_state.remove_for_delegator(&delegator)
		} else {
			auto_compounding_state.set_for_delegator(delegator.clone(), value)
		};
		if state_updated {
			auto_compounding_state.set_storage(&candidate);
		}

		<Pallet<T>>::deposit_event(Event::AutoCompoundSet { candidate, delegator, value });
		let actual_weight = Some(T::WeightInfo::set_auto_compound(
			candidate_auto_compounding_count,
			delegator_state.delegations.0.len() as u32,
		));
		Ok(actual_weight.into())
	}

	/// Removes the auto-compounding value for a delegation. This should be called when the
	/// delegation is revoked to cleanup storage. Storage is only written iff the entry existed.
	pub(crate) fn remove_auto_compound(candidate: &T::AccountId, delegator: &T::AccountId) {
		let mut auto_compounding_state = Self::get_storage(candidate);
		if auto_compounding_state.remove_for_delegator(delegator) {
			auto_compounding_state.set_storage(&candidate);
		}
	}

	/// Returns the value of auto-compound, if it exists for a given delegation, zero otherwise.
	pub(crate) fn auto_compound(candidate: &T::AccountId, delegator: &T::AccountId) -> Percent {
		let delegations_config = Self::get_storage(candidate);
		delegations_config.get_for_delegator(&delegator).unwrap_or_else(Percent::zero)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::Test;

	#[test]
	fn test_set_for_delegator_inserts_config_and_returns_true_if_entry_missing() {
		let mut delegations_config = AutoCompoundDelegations::<Test>::new(vec![]);
		assert!(delegations_config.set_for_delegator(1, Percent::from_percent(50)));
		assert_eq!(
			vec![AutoCompoundConfig { delegator: 1, value: Percent::from_percent(50) }],
			delegations_config.into_inner(),
		);
	}

	#[test]
	fn test_set_for_delegator_updates_config_and_returns_true_if_entry_changed() {
		let mut delegations_config =
			AutoCompoundDelegations::<Test>::new(vec![AutoCompoundConfig {
				delegator: 1,
				value: Percent::from_percent(10),
			}]);
		assert!(delegations_config.set_for_delegator(1, Percent::from_percent(50)));
		assert_eq!(
			vec![AutoCompoundConfig { delegator: 1, value: Percent::from_percent(50) }],
			delegations_config.into_inner(),
		);
	}

	#[test]
	fn test_set_for_delegator_updates_config_and_returns_false_if_entry_unchanged() {
		let mut delegations_config =
			AutoCompoundDelegations::<Test>::new(vec![AutoCompoundConfig {
				delegator: 1,
				value: Percent::from_percent(10),
			}]);
		assert!(!delegations_config.set_for_delegator(1, Percent::from_percent(10)));
		assert_eq!(
			vec![AutoCompoundConfig { delegator: 1, value: Percent::from_percent(10) }],
			delegations_config.into_inner(),
		);
	}

	#[test]
	fn test_remove_for_delegator_returns_false_if_entry_was_missing() {
		let mut delegations_config = AutoCompoundDelegations::<Test>::new(vec![]);
		assert!(!delegations_config.remove_for_delegator(&1));
	}

	#[test]
	fn test_remove_delegation_config_returns_true_if_entry_existed() {
		let mut delegations_config =
			AutoCompoundDelegations::<Test>::new(vec![AutoCompoundConfig {
				delegator: 1,
				value: Percent::from_percent(10),
			}]);
		assert!(delegations_config.remove_for_delegator(&1));
	}
}