
//! Autogenerated weights for `pallet_account_proxy`
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

/// Weight functions for `pallet_account_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_account_proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `p` is `[1, 3]`.
	fn proxy(p: u32, ) -> Weight {
		(48_085_000 as Weight)
			// Standard Error: 191_000
			.saturating_add((56_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		(59_522_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((666_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 385_000
			.saturating_add((1_329_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		(18_332_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((574_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 222_000
			.saturating_add((952_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		(19_408_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((566_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 201_000
			.saturating_add((760_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		(50_098_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((632_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 342_000
			.saturating_add((854_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn add_proxy(p: u32, ) -> Weight {
		(41_510_000 as Weight)
			// Standard Error: 218_000
			.saturating_add((665_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxy(p: u32, ) -> Weight {
		(40_475_000 as Weight)
			// Standard Error: 159_000
			.saturating_add((459_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxies(p: u32, ) -> Weight {
		(17_121_000 as Weight)
			// Standard Error: 116_000
			.saturating_add((436_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn anonymous(_p: u32, ) -> Weight {
		(49_943_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 2]`.
	fn kill_anonymous(_p: u32, ) -> Weight {
		(19_957_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
