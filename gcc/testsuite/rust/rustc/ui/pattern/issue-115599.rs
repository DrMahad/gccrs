const CONST_STRING: String = String::new();

fn main() {
    let empty_str = String::from("");
    if let CONST_STRING = empty_str {}
// { dg-error "" "" { target *-*-* } .-1 }
}

