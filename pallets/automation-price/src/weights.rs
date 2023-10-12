// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_automation_price
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `copvan.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// target/release/oak-collator
// benchmark
// pallet
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_price
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./pallets/automation-price/src/raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

// Summary:
//:initialize_asset_extrinsic 14_442_352
//:asset_price_update_extrinsic 10_694_700
//:schedule_xcmp_task_extrinsic 24_000_000
//:cancel_task_extrinsic 21_000_000
//:run_xcmp_task 28_000_000
//:emit_event 4_000_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_price.
pub trait WeightInfo {
	fn initialize_asset_extrinsic(v: u32, ) -> Weight;
	fn asset_price_update_extrinsic(v: u32, ) -> Weight;
	fn schedule_xcmp_task_extrinsic() -> Weight;
	fn cancel_task_extrinsic() -> Weight;
	fn run_xcmp_task() -> Weight;
	fn emit_event() -> Weight;
    fn remove_task() -> Weight;
}

/// Weights for pallet_automation_price using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: AutomationPrice AssetRegistry (r:1 w:1)
	// Proof Skipped: AutomationPrice AssetRegistry (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 36]`.
	fn initialize_asset_extrinsic(_v: u32, ) -> Weight {
		Weight::from_ref_time(14_442_352 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationPrice AssetRegistry (r:1 w:0)
	// Proof Skipped: AutomationPrice AssetRegistry (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 100]`.
	fn asset_price_update_extrinsic(v: u32, ) -> Weight {
		Weight::from_ref_time(10_694_700 as u64)
			// Standard Error: 982
			.saturating_add(Weight::from_ref_time(135_931 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: AutomationPrice SortedTasksIndex (r:1 w:1)
	// Proof Skipped: AutomationPrice SortedTasksIndex (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice AccountTasks (r:0 w:1)
	// Proof Skipped: AutomationPrice AccountTasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice Tasks (r:0 w:1)
	// Proof Skipped: AutomationPrice Tasks (max_values: None, max_size: None, mode: Measured)
	fn schedule_xcmp_task_extrinsic() -> Weight {
		Weight::from_ref_time(24_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AutomationPrice Tasks (r:1 w:1)
	// Proof Skipped: AutomationPrice Tasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice AccountTasks (r:0 w:1)
	// Proof Skipped: AutomationPrice AccountTasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice SortedTasksIndex (r:0 w:1)
	// Proof Skipped: AutomationPrice SortedTasksIndex (max_values: None, max_size: None, mode: Measured)
	fn cancel_task_extrinsic() -> Weight {
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Proof Skipped: UnknownTokens ConcreteFungibleBalances (max_values: None, max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationToAssetId (max_values: None, max_size: None, mode: Measured)
	fn run_xcmp_task() -> Weight {
		Weight::from_ref_time(28_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
	}
	fn emit_event() -> Weight {
		Weight::from_ref_time(4_000_000 as u64)
	}

	fn remove_task() -> Weight {
		Weight::from_ref_time(4_000_000 as u64)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AutomationPrice AssetRegistry (r:1 w:1)
	// Proof Skipped: AutomationPrice AssetRegistry (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 36]`.
	fn initialize_asset_extrinsic(_v: u32, ) -> Weight {
		Weight::from_ref_time(14_442_352 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationPrice AssetRegistry (r:1 w:0)
	// Proof Skipped: AutomationPrice AssetRegistry (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 100]`.
	fn asset_price_update_extrinsic(v: u32, ) -> Weight {
		Weight::from_ref_time(10_694_700 as u64)
			// Standard Error: 982
			.saturating_add(Weight::from_ref_time(135_931 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: AutomationPrice SortedTasksIndex (r:1 w:1)
	// Proof Skipped: AutomationPrice SortedTasksIndex (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice AccountTasks (r:0 w:1)
	// Proof Skipped: AutomationPrice AccountTasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice Tasks (r:0 w:1)
	// Proof Skipped: AutomationPrice Tasks (max_values: None, max_size: None, mode: Measured)
	fn schedule_xcmp_task_extrinsic() -> Weight {
		Weight::from_ref_time(24_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: AutomationPrice Tasks (r:1 w:1)
	// Proof Skipped: AutomationPrice Tasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice AccountTasks (r:0 w:1)
	// Proof Skipped: AutomationPrice AccountTasks (max_values: None, max_size: None, mode: Measured)
	// Storage: AutomationPrice SortedTasksIndex (r:0 w:1)
	// Proof Skipped: AutomationPrice SortedTasksIndex (max_values: None, max_size: None, mode: Measured)
	fn cancel_task_extrinsic() -> Weight {
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Proof Skipped: UnknownTokens ConcreteFungibleBalances (max_values: None, max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationToAssetId (max_values: None, max_size: None, mode: Measured)
	fn run_xcmp_task() -> Weight {
		Weight::from_ref_time(28_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
	}
	fn emit_event() -> Weight {
		Weight::from_ref_time(4_000_000 as u64)
	}

	fn remove_task() -> Weight {
		Weight::from_ref_time(4_000_000 as u64)
	}
}
