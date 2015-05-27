[![Build Status](https://travis-ci.org/oli-obk/rust-sa.svg?branch=master)](https://travis-ci.org/oli-obk/rust-sa)
[![Latest Version](https://img.shields.io/crates/v/static_assert.svg)](https://crates.io/crates/static_assert)

##Example usage

```rust
#![feature(plugin, custom_attribute)]
#![plugin(static_assert_)]
#[macro_use(static_assert)] extern crate static_assert_;

fn main() {
    #[static_assert_]
    const TEST: bool = false;

    static_assert!(5 == 4);
}
```
