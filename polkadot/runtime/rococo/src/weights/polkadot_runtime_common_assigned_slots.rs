// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_common::assigned_slots`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=runtime_common::assigned_slots
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/runtime_common_assigned_slots.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_common::assigned_slots`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> polkadot_runtime_common::assigned_slots::WeightInfo for WeightInfo<T> {
	/// Storage: `Registrar::Paras` (r:1 w:1)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssignedSlots::PermanentSlots` (r:1 w:1)
	/// Proof: `AssignedSlots::PermanentSlots` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::TemporarySlots` (r:1 w:0)
	/// Proof: `AssignedSlots::TemporarySlots` (`max_values`: None, `max_size`: Some(61), added: 2536, mode: `MaxEncodedLen`)
	/// Storage: `Slots::Leases` (r:1 w:1)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssignedSlots::PermanentSlotCount` (r:1 w:1)
	/// Proof: `AssignedSlots::PermanentSlotCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::MaxPermanentSlots` (r:1 w:0)
	/// Proof: `AssignedSlots::MaxPermanentSlots` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn assign_perm_parachain_slot() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `730`
		//  Estimated: `4195`
		// Minimum execution time: 71_337_000 picoseconds.
		Weight::from_parts(80_807_000, 0)
			.saturating_add(Weight::from_parts(0, 4195))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:1)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssignedSlots::PermanentSlots` (r:1 w:0)
	/// Proof: `AssignedSlots::PermanentSlots` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::TemporarySlots` (r:1 w:1)
	/// Proof: `AssignedSlots::TemporarySlots` (`max_values`: None, `max_size`: Some(61), added: 2536, mode: `MaxEncodedLen`)
	/// Storage: `Slots::Leases` (r:1 w:1)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssignedSlots::TemporarySlotCount` (r:1 w:1)
	/// Proof: `AssignedSlots::TemporarySlotCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::MaxTemporarySlots` (r:1 w:0)
	/// Proof: `AssignedSlots::MaxTemporarySlots` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::ActiveTemporarySlotCount` (r:1 w:1)
	/// Proof: `AssignedSlots::ActiveTemporarySlotCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ActionsQueue` (r:1 w:1)
	/// Proof: `Paras::ActionsQueue` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn assign_temp_parachain_slot() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `730`
		//  Estimated: `4195`
		// Minimum execution time: 60_188_000 picoseconds.
		Weight::from_parts(63_932_000, 0)
			.saturating_add(Weight::from_parts(0, 4195))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `AssignedSlots::PermanentSlots` (r:1 w:0)
	/// Proof: `AssignedSlots::PermanentSlots` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssignedSlots::TemporarySlots` (r:1 w:1)
	/// Proof: `AssignedSlots::TemporarySlots` (`max_values`: None, `max_size`: Some(61), added: 2536, mode: `MaxEncodedLen`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Slots::Leases` (r:1 w:1)
	/// Proof: `Slots::Leases` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AssignedSlots::TemporarySlotCount` (r:1 w:1)
	/// Proof: `AssignedSlots::TemporarySlotCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn unassign_parachain_slot() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `856`
		//  Estimated: `4321`
		// Minimum execution time: 35_764_000 picoseconds.
		Weight::from_parts(38_355_000, 0)
			.saturating_add(Weight::from_parts(0, 4321))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AssignedSlots::MaxPermanentSlots` (r:0 w:1)
	/// Proof: `AssignedSlots::MaxPermanentSlots` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_permanent_slots() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_634_000 picoseconds.
		Weight::from_parts(4_852_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AssignedSlots::MaxTemporarySlots` (r:0 w:1)
	/// Proof: `AssignedSlots::MaxTemporarySlots` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_temporary_slots() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_563_000 picoseconds.
		Weight::from_parts(4_829_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
