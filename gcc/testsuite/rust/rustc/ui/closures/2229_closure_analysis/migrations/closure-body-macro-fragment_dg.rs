//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
#![warn(rust_2021_compatibility)]

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

macro_rules! m {
    (@ $body:expr) => {{
        let f = || $body;
// { dg-warning "" "" { target *-*-* } .-1 }
        f();
    }};
    ($body:block) => {{
        m!(@ $body);
    }};
}

fn main() {
    let a = (Foo(0), Foo(1));
    m!({
// { help "" "" { target *-*-* } .-1 }
        let x = a.0;
        println!("{:?}", x);
    });
}

