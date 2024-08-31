//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

// See #124385 for more details.

trait Foo<'a> {}
fn needs_foo<T>(_: T)
where
    for<'a> Wrap<T>: Foo<'a>,
{
}

struct Wrap<T>(T);
impl<'a, T> Foo<'a> for Wrap<T> where T: Fn(&'a i32) {}

fn main() {
    needs_foo(|x| {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        x.to_string();
    });
}

