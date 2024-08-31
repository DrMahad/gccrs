// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]
#![feature(stmt_expr_attributes)]

// Should capture the discriminant since a variant of a multivariant enum is
// mentioned in the match arm; the discriminant is captured by the closure regardless
// of if it creates a binding
fn test_1_should_capture() {
    let variant = Some(2229);
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match variant {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            Some(_) => {}
            _ => {}
        }
    };
    c();
}

// Should not capture the discriminant since only a wildcard is mentioned in the
// match arm
fn test_2_should_not_capture() {
    let variant = Some(2229);
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match variant {
            _ => {}
        }
    };
    c();
}

// Testing single variant patterns
enum SingleVariant {
    Points(u32)
}

// Should not capture the discriminant since the single variant mentioned
// in the match arm does not trigger a binding
fn test_3_should_not_capture_single_variant() {
    let variant = SingleVariant::Points(1);
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match variant {
            SingleVariant::Points(_) => {}
        }
    };
    c();
}

// Should not capture the discriminant since the single variant mentioned
// in the match arm does not trigger a binding
fn test_6_should_capture_single_variant() {
    let variant = SingleVariant::Points(1);
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match variant {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
            SingleVariant::Points(a) => {
                println!("{:?}", a);
            }
        }
    };
    c();
}

// Should not capture the discriminant since only wildcards are mentioned in the
// match arm
fn test_4_should_not_capture_array() {
    let array: [i32; 3] = [0; 3];
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match array {
            [_,_,_] => {}
        }
    };
    c();

    // We also do not need to capture an array
    // behind a reference (#112607)
    let array: &[i32; 3] = &[0; 3];
    let c = #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match array {
            [_, _, _] => {}
        }
    };
    c();

    // We should still not insert a read if the array is inside an
    // irrefutable pattern
    struct Foo<T>(T);
    let f = &Foo(&[10; 3]);
    let c = #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match f {
            Foo([_, _, _]) => ()
        }
    };
    c();
}

// Testing MultiVariant patterns
enum MVariant {
    A,
    B,
    C,
}

// Should capture the discriminant since a variant of the multi variant enum is
// mentioned in the match arm; the discriminant is captured by the closure
// regardless of if it creates a binding
fn test_5_should_capture_multi_variant() {
    let variant = MVariant::A;
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match variant {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            MVariant::A => {}
            _ => {}
        }
    };
    c();
}

// Even though all patterns are wild, we need to read the discriminant
// in order to test the slice length
fn test_7_should_capture_slice_len() {
    let slice: &[i32] = &[1, 2, 3];
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match slice {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            [_,_,_] => {},
            _ => {}
        }
    };
    c();
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match slice {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            [] => {},
            _ => {}
        }
    };
    c();
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        match slice {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
            [_, .. ,_] => {},
            _ => {}
        }
    };
    c();
}

// Wild pattern that doesn't bind, so no capture
fn test_8_capture_slice_wild() {
    let slice: &[i32] = &[1, 2, 3];
    let c =  #[rustc_capture_analysis]
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        match slice {
            [..] => {},
            _ => {}
        }
    };
    c();
}

fn main() {
    test_1_should_capture();
    test_2_should_not_capture();
    test_3_should_not_capture_single_variant();
    test_6_should_capture_single_variant();
    test_4_should_not_capture_array();
    test_5_should_capture_multi_variant();
    test_7_should_capture_slice_len();
    test_8_capture_slice_wild();
}

