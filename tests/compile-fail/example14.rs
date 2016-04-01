#![feature(plugin)]
#![plugin(static_assert)]

static_assert!(false); //~ ERROR static assertion failed
static_assert!(false); //~ ERROR static assertion failed

fn main() {
}
