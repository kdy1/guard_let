use guard_let::prelude::*;

enum Enum {
    A(String),
    B(usize),
    C,
}

fn eat_string(_: String) {}

#[guard]
fn usage() {
    let v = Enum::A(String::from(""));

    guard_let!(v as Enum::A(s), {
        // Type of v is Enum at here.
        println!("v is not A: {:?}", v);
        return;
    });

    // Type of s is String
    eat_string(s)
}
