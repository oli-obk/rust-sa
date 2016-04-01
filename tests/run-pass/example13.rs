#![feature(plugin)]
#![plugin(static_assert)]

const TEST3: bool = true;

static_assert!(TEST3);

fn main() {
}
