#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]

fn main() {
    #[static_assert_]
    #[allow(dead_code)]
    const TEST: bool = true;

    const X: i32 = 3;
    const TEST3: bool = X < 5;
    #[static_assert_]
    #[allow(dead_code)]
    const TEST2: bool = TEST3;

    const Y: f64 = 3.14;
    const TEST4: bool = Y < 5.1;
    #[static_assert_]
    #[allow(dead_code)]
    const TEST5: bool = TEST4;
}
