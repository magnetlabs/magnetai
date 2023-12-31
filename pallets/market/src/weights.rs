// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-s7kdgajz-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/substrate/.git/.artifacts/bench.json
// --pallet=frame-system
// --chain=dev
// --header=./HEADER-APACHE2
// --output=./frame/system/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for frame_system.
pub trait WeightInfo {
	fn new_order(b: u32, ) -> Weight;
	fn api_ready(b: u32, ) -> Weight;
	fn upload_metrics(b: u32, ) -> Weight;
    fn add_relayer() -> Weight;
}

/// Weights for frame_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: crate::Config> WeightInfo for SubstrateWeight<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn new_order(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_004_000 picoseconds.
		Weight::from_parts(2_119_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(390, 0).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn api_ready(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_455, 0).saturating_mul(b.into()))
	}

	/// The range of component `b` is `[0, 3932160]`.
	fn upload_metrics(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_455, 0).saturating_mul(b.into()))
	}
    fn add_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `b` is `[0, 3932160]`.
	fn new_order(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_004_000 picoseconds.
		Weight::from_parts(2_119_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(390, 0).saturating_mul(b.into()))
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn api_ready(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_455, 0).saturating_mul(b.into()))
	}

	/// The range of component `b` is `[0, 3932160]`.
	fn upload_metrics(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_455, 0).saturating_mul(b.into()))
	}
    fn add_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_032_000 picoseconds.
		Weight::from_parts(8_097_000, 0)
	}
}
