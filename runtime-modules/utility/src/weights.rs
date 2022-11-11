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

//! Autogenerated weights for joystream_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./../target/release/joystream-node
// benchmark
// pallet
// --base-path=/mnt/disks/local-ssd/
// --pallet=joystream_utility
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./../devops/joystream-pallet-weight-template.hbs
// --output=./../runtime-modules/utility/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for joystream_utility.
pub trait WeightInfo {
	fn execute_signal_proposal(_i: u32, ) -> Weight;
	fn update_working_group_budget_positive() -> Weight;
	fn update_working_group_budget_negative() -> Weight;
	fn burn_account_tokens() -> Weight;
}

/// Weights for joystream_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn execute_signal_proposal(i: u32, ) -> Weight {
		(13_391_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((740_000 as Weight).saturating_mul(i as Weight))
	}
	// Storage: Instance1WorkingGroup Budget (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn update_working_group_budget_positive() -> Weight {
		(25_120_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance1WorkingGroup Budget (r:1 w:1)
	// Storage: Council Budget (r:1 w:1)
	fn update_working_group_budget_negative() -> Weight {
		(24_960_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:1 w:1)
	fn burn_account_tokens() -> Weight {
		(37_611_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn execute_signal_proposal(i: u32, ) -> Weight {
		0
	}
	fn update_working_group_budget_positive() -> Weight {
		0
	}
	fn update_working_group_budget_negative() -> Weight {
		0
	}
	fn burn_account_tokens() -> Weight {
		0
	}
}