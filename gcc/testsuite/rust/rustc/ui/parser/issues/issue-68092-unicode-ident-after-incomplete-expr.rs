macro_rules! x {
    ($($c:tt)*) => {
        $($c)ö*
    };
}

fn main() {
    x!(!); // { dg-error "" "" { target *-*-* } }
}

