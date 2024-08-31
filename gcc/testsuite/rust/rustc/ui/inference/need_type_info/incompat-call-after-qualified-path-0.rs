// issue#121613

#![feature(more_qualified_paths)]

struct S {}

struct Foo;

trait A {
    type Assoc;
}

impl A for Foo {
    type Assoc = S;
}

fn f() {}

fn main() {
  <Foo as A>::Assoc {};
  f(|a, b| a.cmp(b));
// { dg-error ".E0061." "" { target *-*-* } .-1 }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
}

