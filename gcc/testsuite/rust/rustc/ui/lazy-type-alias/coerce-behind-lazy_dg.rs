//@ check-pass
//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(lazy_type_alias)]
// { dg-warning "" "" { target *-*-* } .-1 }

use std::any::Any;

type Coerce = Box<dyn Any>;

fn test() -> Coerce {
    Box::new(1)
}

fn main() {}

