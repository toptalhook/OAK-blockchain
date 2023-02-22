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

//! Autogenerated weights for pallet_automation_time
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-22, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `actions-runner-1`, CPU: `Intel(R) Xeon(R) E-2388G CPU @ 3.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// ./oak-collator
// benchmark
// pallet
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_automation_time
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./automation_time-raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

// Summary:
//:schedule_notify_task_empty 74_716_000
//:schedule_notify_task_full 103_069_000
//:schedule_xcmp_task_full 141_518_000
//:schedule_native_transfer_task_empty 73_902_000
//:schedule_native_transfer_task_full 102_604_000
//:schedule_auto_compound_delegated_stake_task_full 114_059_000
//:schedule_dynamic_dispatch_task 71_360_000
//:schedule_dynamic_dispatch_task_full 96_902_000
//:cancel_scheduled_task_full 645_197_000
//:force_cancel_scheduled_task 32_479_000
//:force_cancel_scheduled_task_full 640_543_000
//:run_notify_task 9_960_000
//:run_native_transfer_task 36_209_000
//:run_xcmp_task 93_317_000
//:run_auto_compound_delegated_stake_task 70_034_000
//:run_dynamic_dispatch_action 19_752_000
//:run_dynamic_dispatch_action_fail_decode 10_302_000
//:run_missed_tasks_many_found 210_000
//:run_missed_tasks_many_missing 187_000
//:run_tasks_many_found 232_000
//:run_tasks_many_missing 176_000
//:update_task_queue_overhead 3_437_000
//:append_to_missed_tasks 4_247_000
//:update_scheduled_task_queue 42_676_000
//:shift_missed_tasks 28_294_000
//:migration_add_schedule_to_task 13_337_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_automation_time.
pub trait WeightInfo {
	fn schedule_notify_task_empty() -> Weight;
	fn schedule_notify_task_full(v: u32, ) -> Weight;
	fn schedule_xcmp_task_full(v: u32, ) -> Weight;
	fn schedule_native_transfer_task_empty() -> Weight;
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight;
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight;
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight;
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight;
	fn cancel_scheduled_task_full() -> Weight;
	fn force_cancel_scheduled_task() -> Weight;
	fn force_cancel_scheduled_task_full() -> Weight;
	fn run_notify_task() -> Weight;
	fn run_native_transfer_task() -> Weight;
	fn run_xcmp_task() -> Weight;
	fn run_auto_compound_delegated_stake_task() -> Weight;
	fn run_dynamic_dispatch_action() -> Weight;
	fn run_dynamic_dispatch_action_fail_decode() -> Weight;
	fn run_missed_tasks_many_found(v: u32, ) -> Weight;
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight;
	fn run_tasks_many_found(v: u32, ) -> Weight;
	fn run_tasks_many_missing(v: u32, ) -> Weight;
	fn update_task_queue_overhead() -> Weight;
	fn append_to_missed_tasks(v: u32, ) -> Weight;
	fn update_scheduled_task_queue() -> Weight;
	fn shift_missed_tasks() -> Weight;
	fn migration_add_schedule_to_task(v: u32, ) -> Weight;
}

/// Weights for pallet_automation_time using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_notify_task_empty() -> Weight {
		Weight::from_ref_time(74_716_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(103_069_000 as u64)
			// Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(26_474_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(141_518_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(26_145_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_native_transfer_task_empty() -> Weight {
		Weight::from_ref_time(73_902_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(102_604_000 as u64)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(26_227_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		Weight::from_ref_time(114_059_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		Weight::from_ref_time(71_360_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(3_844_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(96_902_000 as u64)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(26_890_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		Weight::from_ref_time(645_197_000 as u64)
			.saturating_add(T::DbWeight::get().reads(27 as u64))
			.saturating_add(T::DbWeight::get().writes(25 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		Weight::from_ref_time(32_479_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		Weight::from_ref_time(640_543_000 as u64)
			.saturating_add(T::DbWeight::get().reads(27 as u64))
			.saturating_add(T::DbWeight::get().writes(25 as u64))
	}
	fn run_notify_task() -> Weight {
		Weight::from_ref_time(9_960_000 as u64)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		Weight::from_ref_time(36_209_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		Weight::from_ref_time(93_317_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		Weight::from_ref_time(70_034_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		Weight::from_ref_time(19_752_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		Weight::from_ref_time(10_302_000 as u64)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		Weight::from_ref_time(210_000 as u64)
			// Standard Error: 12_000
			.saturating_add(Weight::from_ref_time(19_138_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		Weight::from_ref_time(187_000 as u64)
			// Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(14_925_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		Weight::from_ref_time(232_000 as u64)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(43_933_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		Weight::from_ref_time(176_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(13_811_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		Weight::from_ref_time(3_437_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		Weight::from_ref_time(4_247_000 as u64)
			// Standard Error: 42_000
			.saturating_add(Weight::from_ref_time(1_158_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		Weight::from_ref_time(42_676_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		Weight::from_ref_time(28_294_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		Weight::from_ref_time(13_337_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(8_031_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_notify_task_empty() -> Weight {
		Weight::from_ref_time(74_716_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_notify_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(103_069_000 as u64)
			// Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(26_474_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_xcmp_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(141_518_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(26_145_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_native_transfer_task_empty() -> Weight {
		Weight::from_ref_time(73_902_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_native_transfer_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(102_604_000 as u64)
			// Standard Error: 20_000
			.saturating_add(Weight::from_ref_time(26_227_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	fn schedule_auto_compound_delegated_stake_task_full() -> Weight {
		Weight::from_ref_time(114_059_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(7 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task(v: u32, ) -> Weight {
		Weight::from_ref_time(71_360_000 as u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(3_844_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	/// The range of component `v` is `[1, 24]`.
	fn schedule_dynamic_dispatch_task_full(v: u32, ) -> Weight {
		Weight::from_ref_time(96_902_000 as u64)
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(26_890_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn cancel_scheduled_task_full() -> Weight {
		Weight::from_ref_time(645_197_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(27 as u64))
			.saturating_add(RocksDbWeight::get().writes(25 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn force_cancel_scheduled_task() -> Weight {
		Weight::from_ref_time(32_479_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: AutomationTime LastTimeSlot (r:1 w:0)
	// Storage: AutomationTime ScheduledTasksV3 (r:24 w:24)
	fn force_cancel_scheduled_task_full() -> Weight {
		Weight::from_ref_time(640_543_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(27 as u64))
			.saturating_add(RocksDbWeight::get().writes(25 as u64))
	}
	fn run_notify_task() -> Weight {
		Weight::from_ref_time(9_960_000 as u64)
	}
	// Storage: System Account (r:2 w:2)
	fn run_native_transfer_task() -> Weight {
		Weight::from_ref_time(36_209_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: XcmpHandler XcmChainCurrencyData (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: UnknownTokens ConcreteFungibleBalances (r:1 w:0)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	fn run_xcmp_task() -> Weight {
		Weight::from_ref_time(93_317_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: ParachainStaking DelegatorState (r:1 w:1)
	// Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: ParachainStaking CandidateInfo (r:1 w:1)
	// Storage: ParachainStaking TopDelegations (r:1 w:1)
	// Storage: ParachainStaking CandidatePool (r:1 w:1)
	// Storage: ParachainStaking Total (r:1 w:1)
	fn run_auto_compound_delegated_stake_task() -> Weight {
		Weight::from_ref_time(70_034_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(8 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:0)
	fn run_dynamic_dispatch_action() -> Weight {
		Weight::from_ref_time(19_752_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn run_dynamic_dispatch_action_fail_decode() -> Weight {
		Weight::from_ref_time(10_302_000 as u64)
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_found(v: u32, ) -> Weight {
		Weight::from_ref_time(210_000 as u64)
			// Standard Error: 12_000
			.saturating_add(Weight::from_ref_time(19_138_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_missed_tasks_many_missing(v: u32, ) -> Weight {
		Weight::from_ref_time(187_000 as u64)
			// Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(14_925_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_found(v: u32, ) -> Weight {
		Weight::from_ref_time(232_000 as u64)
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(43_933_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(v as u64)))
	}
	// Storage: AutomationTime AccountTasks (r:1 w:0)
	/// The range of component `v` is `[0, 1]`.
	fn run_tasks_many_missing(v: u32, ) -> Weight {
		Weight::from_ref_time(176_000 as u64)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(13_811_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn update_task_queue_overhead() -> Weight {
		Weight::from_ref_time(3_437_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
	}
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	/// The range of component `v` is `[0, 2]`.
	fn append_to_missed_tasks(v: u32, ) -> Weight {
		Weight::from_ref_time(4_247_000 as u64)
			// Standard Error: 42_000
			.saturating_add(Weight::from_ref_time(1_158_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationTime TaskQueueV2 (r:1 w:1)
	// Storage: AutomationTime MissedQueueV2 (r:1 w:1)
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn update_scheduled_task_queue() -> Weight {
		Weight::from_ref_time(42_676_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: AutomationTime ScheduledTasksV3 (r:1 w:1)
	fn shift_missed_tasks() -> Weight {
		Weight::from_ref_time(28_294_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: AutomationTime AccountTasks (r:2 w:1)
	/// The range of component `v` is `[1, 100]`.
	fn migration_add_schedule_to_task(v: u32, ) -> Weight {
		Weight::from_ref_time(13_337_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(8_031_000 as u64).saturating_mul(v as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(v as u64)))
	}
}
