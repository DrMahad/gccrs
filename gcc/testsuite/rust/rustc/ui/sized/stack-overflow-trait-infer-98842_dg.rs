// #98842 stack overflow in trait inference
// issue: rust-lang/rust#98842
//@ check-fail
// { dg-additional-options "-frust-edition=2021" }
//@ stderr-per-bitwidth
// { dg-error "" "" { target *-*-* } .-5 }

// If the inner `Foo` is named through an associated type,
// the "infinite size" error does not occur.
struct Foo(<&'static Foo as ::core::ops::Deref>::Target);
// But Rust will be unable to know whether `Foo` is sized or not,
// and it will infinitely recurse somewhere trying to figure out the
// size of this pointer (is my guess):
const _: *const Foo = 0 as _;
// { dg-error "" "" { target *-*-* } .-1 }

pub fn main() {}

