// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-b3zmxxc-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/cumulus/.git/.artifacts/bench.json
// --pallet=pallet_utility
// --chain=bridge-hub-polkadot-dev
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 12_876 nanoseconds.
		Weight::from_ref_time(15_632_636)
			// Standard Error: 1_443
			.saturating_add(Weight::from_ref_time(4_450_173).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 6_177 nanoseconds.
		Weight::from_ref_time(6_434_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 13_062 nanoseconds.
		Weight::from_ref_time(18_049_724)
			// Standard Error: 1_803
			.saturating_add(Weight::from_ref_time(4_699_471).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 14_760 nanoseconds.
		Weight::from_ref_time(15_538_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 12_837 nanoseconds.
		Weight::from_ref_time(16_664_335)
			// Standard Error: 1_252
			.saturating_add(Weight::from_ref_time(4_427_910).saturating_mul(c.into()))
	}
	fn ensure_dispatch_as() -> Weight {
		// Minimum execution time: 14_760 nanoseconds.
		Weight::from_ref_time(15_538_000)
	}
}