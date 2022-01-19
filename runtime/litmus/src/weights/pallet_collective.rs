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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-18, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("generate-litmus"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// --chain=generate-litmus
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_collective
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((22_060_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 18_000
			.saturating_add((154_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 18_000
			.saturating_add((28_229_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		(34_541_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 3_000
			.saturating_add((116_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		(39_616_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((221_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		(41_729_000 as Weight)
			// Standard Error: 0
			.saturating_add((10_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((118_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((526_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		(49_987_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((311_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		(49_915_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((229_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((435_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(59_617_000 as Weight)
			// Standard Error: 0
			.saturating_add((16_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 3_000
			.saturating_add((287_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 3_000
			.saturating_add((612_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		(55_122_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((230_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((432_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: ExtrinsicFilter Mode (r:1 w:0)
	// Storage: ExtrinsicFilter BlockedExtrinsics (r:2 w:0)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		(61_200_000 as Weight)
			// Standard Error: 0
			.saturating_add((14_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 3_000
			.saturating_add((312_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 3_000
			.saturating_add((620_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		(31_398_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((496_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}