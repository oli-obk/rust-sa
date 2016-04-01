#![feature(plugin)]
#![plugin(static_assert)]

fn main() {
    static_assert!(5 != 5); //~ ERROR: static assertion failed
}
