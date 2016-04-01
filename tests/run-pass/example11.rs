#![feature(plugin)]
#![plugin(static_assert)]

const Y: f64 = 3.14;
const TEST4: bool = Y < 5.1;

static_assert!(TEST4);

fn main() {
}
