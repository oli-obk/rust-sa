#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]

fn main() {
    #[static_assert_]
    #[allow(dead_code)]
    const TEST: bool = false; //~ ERROR: static assertion failed
}
