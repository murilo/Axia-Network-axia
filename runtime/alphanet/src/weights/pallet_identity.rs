// Copyright 2017-2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 128

// Executed Command:
// target/release/axia
// benchmark
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/alphanet/src/weights/pallet_identity.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	fn add_registrar(r: u32, ) -> Weight {
		(19_890_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((233_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(44_464_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((229_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((898_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	fn set_subs_new(s: u32, ) -> Weight {
		(37_386_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((5_450_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	fn set_subs_old(p: u32, ) -> Weight {
		(36_308_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_670_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(44_814_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((70_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((1_689_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 1_000
			.saturating_add((494_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(47_306_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((202_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((949_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(41_959_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((173_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 1_000
			.saturating_add((951_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fee(r: u32, ) -> Weight {
		(8_008_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((219_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_account_id(r: u32, ) -> Weight {
		(8_096_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((215_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:1)
	fn set_fields(r: u32, ) -> Weight {
		(8_173_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((209_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(31_556_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((159_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((945_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(50_293_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((108_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_681_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn add_sub(s: u32, ) -> Weight {
		(48_580_000 as Weight)
			// Standard Error: 0
			.saturating_add((175_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	fn rename_sub(s: u32, ) -> Weight {
		(15_109_000 as Weight)
			// Standard Error: 0
			.saturating_add((21_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn remove_sub(s: u32, ) -> Weight {
		(49_063_000 as Weight)
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	fn quit_sub(s: u32, ) -> Weight {
		(31_158_000 as Weight)
			// Standard Error: 0
			.saturating_add((153_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
