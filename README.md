# Enumeration of primitive integer kinds

This crate offers [`PrimIntKind`](https://docs.rs/prim_int_kind/latest/prim_int_kind/enum.PrimIntKind.html) enum whose variants represent kinds of primitive integers.

According to Rust's reference, [primitive numeric integer types][primitive numeric type] in Rust are such:

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

`usize` and `isize` are at least 16-bits wide.

> **Note**: Many pieces of Rust code may assume that pointers, `usize`, and
> `isize` are either 32-bit or 64-bit. As a consequence, 16-bit
> pointer support is limited and may require explicit care and acknowledgment
> from a library to support.

# Examples

Stable Rust

```rust
use prim_int_kind::PrimIntKind;

let res_kind = "u16".parse();
assert_eq!(res_kind, Ok(PrimIntKind::U16));
```

With `const_trait_impl` nightly feature

Cargo.toml

```toml
# Read more about features here:
# https://doc.rust-lang.org/cargo/reference/features.html
[features]
const_trait_impl = ["prim_int_kind/const_trait_impl"]
```

src/main.rs

```rust
#![cfg_attr(any(doc,test,doctest, feature = "const_trait_impl"), feature(const_trait_impl))]
// Run via `cargo run --features const_trait_impl`

use core::str::FromStr;
use prim_int_kind::{PrimIntKind, PrimIntKindParsingError};

fn main() {
    #[cfg(any(doc,test,doctest, feature = "const_trait_impl"))]
    const res_kind: Result<PrimIntKind, PrimIntKindParsingError> = PrimIntKind::from_str("u16");
    #[cfg(not(any(doc,test,doctest, feature = "const_trait_impl")))]
    unimplemented!();
    assert_eq!(res_kind, Ok(PrimIntKind::U16));
}
```

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[primitive numeric type]: https://doc.rust-lang.org/reference/types/numeric.html