#![feature(plugin)]
#![plugin(static_assert)]

const TEST3: bool = false;

static_assert!(TEST3); //~ ERROR static assertion failed

fn main() {
}
