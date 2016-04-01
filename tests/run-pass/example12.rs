#![feature(plugin)]
#![plugin(static_assert)]

const X: i32 = 3;
const TEST3: bool = X < 5;

static_assert!(TEST3);

fn main() {
}
