#![doc = include_str!("../README.md")]
#![cfg_attr(
    any(doc, test, doctest, feature = "const_trait_impl"),
    feature(const_trait_impl)
)]

mod from_str;
mod to_prim_int_kind;

pub use from_str::PrimIntKindParsingError;
pub use to_prim_int_kind::ToPrimIntKind;

/// Enumerates primitive integer kinds as per
/// [Rust's reference](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
///
/// # Examples
///
/// Stable Rust
///
/// ```
/// use prim_int_kind::PrimIntKind;
///
/// let res_kind = "u16".parse();
/// assert_eq!(res_kind, Ok(PrimIntKind::U16));
/// ```
///
/// With `const_trait_impl` nightly feature
///
/// Cargo.toml
///
/// ```toml
/// # Read more about features here:
/// # https://doc.rust-lang.org/cargo/reference/features.html
/// [features]
/// const_trait_impl = ["prim_int_kind/const_trait_impl"]
/// ```
///
/// src/main.rs
///
/// ```
/// #![cfg_attr(any(doc,test,doctest, feature = "const_trait_impl"), feature(const_trait_impl))]
/// // Run via `cargo run --features const_trait_impl`
///
/// use core::str::FromStr;
/// use prim_int_kind::{PrimIntKind, PrimIntKindParsingError};
///
/// #[cfg(any(doc,test,doctest, feature = "const_trait_impl"))]
/// const res_kind: Result<PrimIntKind, PrimIntKindParsingError> = PrimIntKind::from_str("u16");
/// #[cfg(not(any(doc,test,doctest, feature = "const_trait_impl")))]
/// let res_kind: Result<PrimIntKind, PrimIntKindParsingError> = PrimIntKind::from_str("u16");
/// assert_eq!(res_kind, Ok(PrimIntKind::U16));
/// ```
#[derive(PartialEq, Eq, Debug)]
pub enum PrimIntKind {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
}
