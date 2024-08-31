//@ check-pass

#![feature(generic_const_exprs)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub fn y<'a, U: 'a>() -> impl IntoIterator<Item = impl IntoIterator<Item = [u8; { 1 + 2 }]> + 'a> {
    [[[1, 2, 3]]]
}
// Make sure that the `predicates_of` for `{ 1 + 2 }` don't mention the duplicated lifetimes of
// the *outer* iterator. Whether they should mention the duplicated lifetimes of the *inner*
// iterator are another question, but not really something we need to answer immediately.

fn main() {}

