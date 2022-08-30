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

//! Autogenerated weights for `parachain_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-12, STEPS: `5`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litentry-dev"), DB CACHE: 20

// Executed Command:
// ./target/release/litentry-collator
// benchmark
// pallet
// --chain=litentry-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=parachain_staking
// --extrinsic=*
// --heap-pages=4096
// --steps=5
// --repeat=10
// --header=./LICENSE_HEADER
// --output=./runtime/litentry/src/weights/parachain_staking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `parachain_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::WeightInfo for WeightInfo<T> {
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_staking_expectations() -> Weight {
		(15_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_inflation() -> Weight {
		(43_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_account() -> Weight {
		(14_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	fn set_parachain_bond_reserve_percent() -> Weight {
		(14_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking TotalSelected (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn set_total_selected() -> Weight {
		(18_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CollatorCommission (r:1 w:1)
	fn set_collator_commission() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking InflationConfig (r:1 w:1)
	fn set_blocks_per_round() -> Weight {
		(23_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn add_candidates_whitelist(x: u32, ) -> Weight {
		(76_974_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((112_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn remove_candidates_whitelist(x: u32, ) -> Weight {
		(76_974_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((112_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:0 w:1)
	// Storage: ParachainStaking CollatorReserveToLockMigrations (r:0 w:1)
	// Storage: ParachainStaking BottomDelegations (r:0 w:1)
	fn join_candidates(x: u32, ) -> Weight {
		(89_407_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((138_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn schedule_leave_candidates(x: u32, ) -> Weight {
		(67_029_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((96_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: Balances Locks (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking BottomDelegations (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_leave_candidates(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 637_000
			.saturating_add((41_134_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn cancel_leave_candidates(x: u32, ) -> Weight {
		(60_859_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((109_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_offline() -> Weight {
		(24_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn go_online() -> Weight {
		(23_400_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn candidate_bond_more() -> Weight {
		(48_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_candidate_bond_less() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	fn execute_candidate_bond_less() -> Weight {
		(88_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	fn cancel_candidate_bond_less() -> Weight {
		(19_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: ParachainStaking DelegatorReserveToLockMigrations (r:0 w:1)
	fn delegate(_x: u32, y: u32, ) -> Weight {
		(180_632_000 as Weight)
			// Standard Error: 27_000
			.saturating_add((35_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn schedule_leave_delegators() -> Weight {
		(30_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn execute_leave_delegators(x: u32, ) -> Weight {
		(22_286_000 as Weight)
			// Standard Error: 512_000
			.saturating_add((33_062_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_leave_delegators() -> Weight {
		(30_900_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_revoke_delegation() -> Weight {
		(29_300_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn delegator_bond_more() -> Weight {
		(67_200_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	fn schedule_delegator_bond_less() -> Weight {
		(34_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_revoke_delegation() -> Weight {
		(126_800_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	// Storage: ParachainStaking Round (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn execute_delegator_bond_less() -> Weight {
		(122_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_revoke_delegation() -> Weight {
		(26_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	fn cancel_delegator_bond_less() -> Weight {
		(65_600_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ParachainStaking Round (r:1 w:1)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking Staked (r:1 w:2)
	// Storage: ParachainStaking InflationConfig (r:1 w:0)
	// Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
	// Storage: System Account (r:302 w:301)
	// Storage: ParachainStaking CollatorCommission (r:1 w:0)
	// Storage: ParachainStaking CandidatePool (r:1 w:0)
	// Storage: ParachainStaking TotalSelected (r:1 w:0)
	// Storage: ParachainStaking CandidateInfo (r:9 w:0)
	// Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
	// Storage: ParachainStaking TopDelegations (r:9 w:0)
	// Storage: ParachainStaking Total (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:10)
	// Storage: ParachainStaking SelectedCandidates (r:0 w:1)
	// Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 4_678_000
			.saturating_add((51_652_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 14_000
			.saturating_add((336_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
	// Storage: ParachainStaking DelayedPayouts (r:1 w:0)
	// Storage: ParachainStaking Points (r:1 w:0)
	// Storage: ParachainStaking AwardedPts (r:2 w:1)
	// Storage: ParachainStaking AtStake (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn pay_one_collator_reward(y: u32, ) -> Weight {
		(47_782_000 as Weight)
			// Standard Error: 162_000
			.saturating_add((17_751_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
	// Storage: ParachainStaking Round (r:1 w:0)
	fn base_on_initialize() -> Weight {
		(4_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}