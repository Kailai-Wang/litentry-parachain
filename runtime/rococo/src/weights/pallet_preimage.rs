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

//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-01, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8259CL CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=rococo-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_preimage
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/rococo/src/weights/pallet_preimage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 46_981 nanoseconds.
		Weight::from_ref_time(47_908_000 as u64)
			// Standard Error: 17
			.saturating_add(Weight::from_ref_time(3_139 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 32_536 nanoseconds.
		Weight::from_ref_time(33_080_000 as u64)
			// Standard Error: 11
			.saturating_add(Weight::from_ref_time(3_139 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 30_280 nanoseconds.
		Weight::from_ref_time(131_283_764 as u64)
			// Standard Error: 44
			.saturating_add(Weight::from_ref_time(3_152 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_preimage() -> Weight {
		// Minimum execution time: 96_195 nanoseconds.
		Weight::from_ref_time(101_930_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_no_deposit_preimage() -> Weight {
		// Minimum execution time: 75_951 nanoseconds.
		Weight::from_ref_time(82_057_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_preimage() -> Weight {
		// Minimum execution time: 69_749 nanoseconds.
		Weight::from_ref_time(74_967_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_no_deposit_preimage() -> Weight {
		// Minimum execution time: 42_326 nanoseconds.
		Weight::from_ref_time(47_541_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_unnoted_preimage() -> Weight {
		// Minimum execution time: 47_283 nanoseconds.
		Weight::from_ref_time(50_413_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_requested_preimage() -> Weight {
		// Minimum execution time: 16_430 nanoseconds.
		Weight::from_ref_time(18_393_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_preimage() -> Weight {
		// Minimum execution time: 71_071 nanoseconds.
		Weight::from_ref_time(75_480_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_unnoted_preimage() -> Weight {
		// Minimum execution time: 16_856 nanoseconds.
		Weight::from_ref_time(18_185_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Minimum execution time: 17_976 nanoseconds.
		Weight::from_ref_time(19_547_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
