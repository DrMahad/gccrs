// Regression test for issue #93210.

//@ aux-crate:doc_hidden_fields=doc-hidden-fields.rs
// { dg-additional-options "-frust-edition= 2021" }

#[derive(Default)]
pub struct A {
    #[doc(hidden)]
    pub hello: i32,
    pub bye: i32,
}

#[derive(Default)]
pub struct C {
    pub hello: i32,
    pub bye: i32,
}

fn main() {
    // We want to list the field `hello` despite being marked
    // `doc(hidden)` because it's defined in this crate.
    A::default().hey;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { dg-note ".E0609." "" { target *-*-* } .-2 }
// { dg-note ".E0609." "" { target *-*-* } .-3 }

    // Here we want to hide the field `hello` since it's marked
    // `doc(hidden)` and comes from an external crate.
    doc_hidden_fields::B::default().hey;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { dg-note ".E0609." "" { target *-*-* } .-2 }
// { dg-note ".E0609." "" { target *-*-* } .-3 }

    C::default().hey;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { dg-note ".E0609." "" { target *-*-* } .-2 }
// { dg-note ".E0609." "" { target *-*-* } .-3 }
}

