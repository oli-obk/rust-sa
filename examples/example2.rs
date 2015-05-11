#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]

fn main() {
    #[static_assert_]
    const TEST: bool = false;
}
