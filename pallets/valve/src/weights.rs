
//! Autogenerated weights for `pallet_valve`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-02, STEPS: `20`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/oak-collator
// benchmark pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_valve
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./pallets/valve/src/raw-weights.rs

// Summary:
//:close_pallet_gate_existing 8_111_000
//:close_pallet_gate_new 19_966_000
//:close_valve 14_566_000
//:open_pallet_gate 20_384_000
//:open_pallet_gates 21_242_000
//:open_valve 15_296_000
//:start_scheduled_tasks 15_020_000
//:stop_scheduled_tasks 14_542_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vest.
pub trait WeightInfo {
	fn close_valve() -> Weight;
	fn open_valve() -> Weight;
	fn close_pallet_gate_new() -> Weight;
	fn close_pallet_gate_existing() -> Weight;
	fn open_pallet_gate() -> Weight;
	fn open_pallet_gates() -> Weight;
	fn stop_scheduled_tasks() -> Weight;
	fn start_scheduled_tasks() -> Weight;
}

/// Weights for pallet_valve using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Valve ValveClosed (r:1 w:1)
	fn close_valve() -> Weight {
		(14_566_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:1)
	fn open_valve() -> Weight {
		(15_296_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn close_pallet_gate_new() -> Weight {
		(19_966_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	fn close_pallet_gate_existing() -> Weight {
		(8_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn open_pallet_gate() -> Weight {
		(20_384_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	// Storage: Valve ClosedPallets (r:0 w:5)
	fn open_pallet_gates() -> Weight {
		(21_242_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn stop_scheduled_tasks() -> Weight {
		(14_542_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn start_scheduled_tasks() -> Weight {
		(15_020_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}


// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Valve ValveClosed (r:1 w:1)
	fn close_valve() -> Weight {
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:1)
	fn open_valve() -> Weight {
		(13_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn close_pallet_gate_new() -> Weight {
		(17_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	fn close_pallet_gate_existing() -> Weight {
		(5_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn open_pallet_gate() -> Weight {
		(18_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	// Storage: Valve ClosedPallets (r:0 w:5)
	fn open_pallet_gates() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn stop_scheduled_tasks() -> Weight {
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn start_scheduled_tasks() -> Weight {
		(12_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
