[![](https://img.shields.io/crates/v/multi-eq)](https://crates.io/crates/multi-eq)
[![](https://docs.rs/multi-eq/badge.svg)](https://docs.rs/multi-eq)

# `multi_eq`
`multi_eq` is a macro library for creating custom equality trait derives.

```rust
/// Custom comparison trait `CustomEq` with a method `custom_eq`
multi_eq_make_trait!(CustomEq, custom_eq);

#[derive(CustomEq)]
struct MyStruct {
  // Use `PartialEq` to compare this field
  #[custom_eq(cmp = "eq")]
  a: u32,

  // Ignore value of this field when checking equality
  #[custom_eq(ignore)]
  b: bool,
}
```

For more information, see the [documentation](https://docs.rs/multi-eq/).
