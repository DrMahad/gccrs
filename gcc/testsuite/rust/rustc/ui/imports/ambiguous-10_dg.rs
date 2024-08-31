//@ check-pass
// https://github.com/rust-lang/rust/pull/113099#issuecomment-1637022296

mod a {
    pub enum Token {}
}

mod b {
    use crate::a::*;
    #[derive(Debug)]
    pub enum Token {}
}

use crate::a::*;
use crate::b::*;
fn c(_: Token) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
fn main() { }

