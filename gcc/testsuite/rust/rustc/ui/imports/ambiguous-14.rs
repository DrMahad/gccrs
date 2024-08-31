//@ check-pass
// https://github.com/rust-lang/rust/issues/98467

mod a {
    pub fn foo() {}
}

mod b {
    pub fn foo() {}
}

mod f {
    pub use a::*;
    pub use b::*;
}

mod g {
    pub use a::*;
    pub use f::*;
}

fn main() {
    g::foo();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

