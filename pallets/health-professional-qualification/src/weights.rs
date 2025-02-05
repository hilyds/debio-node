//! Autogenerated weights for health_professional_qualification_benchmarking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=health-professional-qualification-benchmarking
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/health-professional-qualification/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for health_professional_qualification_benchmarking.
pub trait WeightInfo { 
	fn create() -> Weight; 
	fn update() -> Weight; 
	fn delete() -> Weight; 
}

/// Weights for health_professional_qualification_benchmarking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: HealthProfessional HealthProfessionals (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCount (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCountByOwner (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:0 w:1) 
	fn create() -> Weight { 
		53_300_000_u64 
			.saturating_add(T::DbWeight::get().reads(3_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:1 w:1) 
	fn update() -> Weight { 
		37_600_000_u64 
			.saturating_add(T::DbWeight::get().reads(1_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:1 w:1) 
	// Storage: HealthProfessional HealthProfessionals (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCount (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCountByOwner (r:1 w:1) 
	fn delete() -> Weight { 
		48_800_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: HealthProfessional HealthProfessionals (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCount (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCountByOwner (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:0 w:1) 
	fn create() -> Weight { 
		53_300_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:1 w:1) 
	fn update() -> Weight { 
		37_600_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: HealthProfessionalQualification HealthProfessionalQualifications (r:1 w:1) 
	// Storage: HealthProfessional HealthProfessionals (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCount (r:1 w:1) 
	// Storage: HealthProfessionalQualification HealthProfessionalQualificationCountByOwner (r:1 w:1) 
	fn delete() -> Weight { 
		48_800_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
}
