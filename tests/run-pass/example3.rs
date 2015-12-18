#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]
#[macro_use(static_assert)] extern crate static_assert_;

fn main() {
    static_assert!(5 == 5);
}
