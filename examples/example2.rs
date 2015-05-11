#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]

fn main() {
    #[static_assert_]
    #[warn(static_assert)]
    #[allow(dead_code)]
    const TEST: bool = false;
}
