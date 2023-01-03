//! Autogenerated weights for menstrual_subscription
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --pallet=menstrual-subscription
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/menstrual-subscription/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for menstrual_subscription.
pub trait WeightInfo { 
	fn add_menstrual_subscription() -> Weight; 
	fn change_menstrual_subscription_status() -> Weight; 
	fn set_menstrual_subscription_paid() -> Weight; 
	fn set_menstrual_subscription_price() -> Weight; 
}

/// Weights for menstrual_subscription using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionByOwner (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionCount (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:0 w:1) 
	fn add_menstrual_subscription() -> Weight { 
		67_600_000_u64 
			.saturating_add(T::DbWeight::get().reads(5_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: MenstrualSubscription AdminKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualSubscription ActiveSubscriptionByOwner (r:0 w:1) 
	fn change_menstrual_subscription_status() -> Weight { 
		52_400_000_u64 
			.saturating_add(T::DbWeight::get().reads(3_u64)) 
			.saturating_add(T::DbWeight::get().writes(2_u64)) 
	}
	// Storage: MenstrualSubscription TreasuryKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:1 w:0) 
	// Storage: System Account (r:1 w:1) 
	// Storage: MenstrualSubscription ActiveSubscriptionByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn set_menstrual_subscription_paid() -> Weight { 
		175_200_000_u64 
			.saturating_add(T::DbWeight::get().reads(6_u64)) 
			.saturating_add(T::DbWeight::get().writes(3_u64)) 
	}
	// Storage: MenstrualSubscription AdminKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:0 w:1) 
	fn set_menstrual_subscription_price() -> Weight { 
		32_500_000_u64 
			.saturating_add(T::DbWeight::get().reads(1_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionByOwner (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionCount (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:0 w:1) 
	fn add_menstrual_subscription() -> Weight { 
		67_600_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: MenstrualSubscription AdminKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: MenstrualSubscription ActiveSubscriptionByOwner (r:0 w:1) 
	fn change_menstrual_subscription_status() -> Weight { 
		52_400_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64)) 
			.saturating_add(RocksDbWeight::get().writes(2_u64)) 
	} 
	// Storage: MenstrualSubscription TreasuryKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionById (r:1 w:1) 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:1 w:0) 
	// Storage: System Account (r:1 w:1) 
	// Storage: MenstrualSubscription ActiveSubscriptionByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn set_menstrual_subscription_paid() -> Weight { 
		175_200_000_u64
			.saturating_add(RocksDbWeight::get().reads(6_u64)) 
			.saturating_add(RocksDbWeight::get().writes(3_u64)) 
	} 
	// Storage: MenstrualSubscription AdminKey (r:1 w:0) 
	// Storage: MenstrualSubscription MenstrualSubscriptionPrices (r:0 w:1) 
	fn set_menstrual_subscription_price() -> Weight { 
		32_500_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
}
