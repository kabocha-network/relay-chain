// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_parachains::configuration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::configuration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/runtime_parachains_configuration.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::configuration`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::configuration::WeightInfo for WeightInfo<T> {
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Configuration BypassConsistencyCheck (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_block_number() -> Weight {
		(10_497_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Configuration BypassConsistencyCheck (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_u32() -> Weight {
		(10_125_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Configuration BypassConsistencyCheck (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_option_u32() -> Weight {
		(10_382_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Configuration BypassConsistencyCheck (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_weight() -> Weight {
		(10_402_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn set_hrmp_open_request_ttl() -> Weight {
		(2_000_000_000_000 as Weight)
	}
	// Storage: Configuration PendingConfigs (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Configuration BypassConsistencyCheck (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	fn set_config_with_balance() -> Weight {
		(10_440_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
