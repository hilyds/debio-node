//! Autogenerated weights for menstrual_calendar
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-13, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=menstrual-calendar
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/menstrual-calendar/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for menstrual_calendar.
pub trait WeightInfo { 
	fn add_menstrual_calendar() -> Weight; 
	fn update_menstrual_calendar() -> Weight; 
	fn add_menstrual_cycle_log() -> Weight; 
	fn update_menstrual_cycle_log() -> Weight; 
	fn remove_menstrual_cycle_log() -> Weight; 
}

/// Weights for menstrual_calendar using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCalendarCountByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:0 w:1) 
	fn add_menstrual_calendar() -> Weight { 
		82_900_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_calendar() -> Weight { 
		52_300_000_u64 
			.saturating_add(T::DbWeight::get().reads(2_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:0 w:1) 
	fn add_menstrual_cycle_log() -> Weight { 
		94_700_000_u64 
			.saturating_add(T::DbWeight::get().reads(5_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_cycle_log() -> Weight { 
		67_900_000_u64 
			.saturating_add(T::DbWeight::get().reads(3_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCountByOwner (r:1 w:1) 
	fn remove_menstrual_cycle_log() -> Weight { 
		93_600_000_u64 
			.saturating_add(T::DbWeight::get().reads(5_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCalendarCountByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:0 w:1) 
	fn add_menstrual_calendar() -> Weight { 
		82_900_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_calendar() -> Weight { 
		52_300_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:0 w:1) 
	fn add_menstrual_cycle_log() -> Weight { 
		94_700_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_menstrual_cycle_log() -> Weight { 
		67_900_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: MenstrualCalendar MenstrualCalendarById (r:1 w:0) 
	// Storage: MenstrualCalendar MenstrualCycleLogById (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogByOwner (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCount (r:1 w:1) 
	// Storage: MenstrualCalendar MenstrualCycleLogCountByOwner (r:1 w:1) 
	fn remove_menstrual_cycle_log() -> Weight { 
		93_600_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
}
