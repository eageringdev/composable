
//! Autogenerated weights for `dex_router`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ad1a4849983a`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/xqbipr3p6p4myn1blgznhki07d6sm1p8-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `dex_router`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> dex_router::WeightInfo for WeightInfo<T> {
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: DexRouter DexRoutes (r:2 w:1)
	fn update_route() -> Weight {
		(96_495_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DexRouter DexRoutes (r:2 w:0)
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: Tokens Accounts (r:13 w:13)
	// Storage: System Account (r:4 w:0)
	// Storage: Pablo PriceCumulativeState (r:4 w:4)
	fn exchange() -> Weight {
		(575_474_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: Tokens Accounts (r:13 w:13)
	// Storage: System Account (r:4 w:0)
	// Storage: Pablo PriceCumulativeState (r:4 w:4)
	fn buy() -> Weight {
		(612_090_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:4 w:0)
	// Storage: Tokens Accounts (r:13 w:13)
	// Storage: System Account (r:4 w:0)
	// Storage: Pablo PriceCumulativeState (r:4 w:4)
	fn sell() -> Weight {
		(576_128_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn add_liquidity() -> Weight {
		(250_585_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: DexRouter DexRoutes (r:1 w:0)
	// Storage: Pablo Pools (r:1 w:0)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:0)
	// Storage: Pablo PriceCumulativeState (r:1 w:1)
	fn remove_liquidity() -> Weight {
		(227_603_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
}
