#![feature(pattern_types, rustc_attrs)]
#![feature(core_pattern_type)]
#![feature(core_pattern_types)]
#![allow(incomplete_features)]

//! check that pattern types don't have an `Add` impl.

use std::pat::pattern_type;

type Y = pattern_type!(u32 is 1..);
type Z = Option<pattern_type!(u32 is 1..)>;

fn main() {
    let x: Y = unsafe { std::mem::transmute(42_u32) };
    let x = x + 1_u32; // { dg-error ".E0369." "" { target *-*-* } }
}

