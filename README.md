# guard_let

[![doc.rs](https://docs.rs/guard_let/badge.svg)](https://docs.rs/guard_let)
[![crates.io](https://img.shields.io/crates/v/guard_let.svg)](https://crates.io/crates/guard_let)

Guard let for rust. In rust, there exists a syntax sugar called if let.
But when it's nested, it make reading code really hard. `guard_let!` does inverse of lf let.
Execute code *below* guard only if guard matches,
and executes provided block if guard does not matches.


# Usage

```rust

use guard_let::guard_let;

enum Enum {
    A(String),
    B(usize),
    C(Struct),
}

struct Struct {
    foo: String,
}

fn eat_string(_: String) {}

#[guard_let]
fn simple_ident() {
    let v = Enum::A(String::from(""));

    guard_let!(v as Enum::A(s), {
        // Type of v is Enum at here.
        println!("v is not A: {:?}", v);
        return;
    });

    // Type of s is String
    eat_string(s)
}

#[guard_let]
fn pattern() {
    let v = Enum::A(String::from(""));

    guard_let!(v as Enum::C(Struct { foo }), {
        // Type of v is Enum at here.
        println!("v is not C: {:?}", v);
        return;
    });

    // Type of s is String
    eat_string(foo)
}


```


# License
MIT