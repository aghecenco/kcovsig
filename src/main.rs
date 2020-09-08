extern crate foobar;

use foobar::*;

fn main() {
    let fb = Foobar {
        foo: 42,
        bar: "asdf".to_string(),
    };
    println!("Hello, world!");
    println!("{:?}", fb);
}
