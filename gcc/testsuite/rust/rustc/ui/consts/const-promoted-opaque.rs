//@revisions: string unit atomic
#![feature(type_alias_impl_trait)]

//! Check that we do not cause cycle errors when trying to
//! obtain information about interior mutability of an opaque type.
//! This used to happen, because when the body-analysis failed, we
//! checked the type instead, but the constant was also defining the
//! hidden type of the opaque type. Thus we ended up relying on the
//! result of our analysis to compute the result of our analysis.

//@[unit] check-pass

mod helper {
    pub type Foo = impl Sized;

    #[cfg(string)]
    pub const FOO: Foo = String::new();

    #[cfg(atomic)]
    pub const FOO: Foo = std::sync::atomic::AtomicU8::new(42);

    #[cfg(unit)]
    pub const FOO: Foo = ();
}
use helper::*;

const BAR: () = {
    let _: &'static _ = &FOO;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
};

const BAZ: &Foo = &FOO;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    let _: &'static _ = &FOO;
// { dg-error "" "" { target *-*-* } .-1 }
}

