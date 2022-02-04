#![doc = include_str!("../README.md")]
#![cfg_attr(any(doc,test,doctest, feature = "const_trait_impl"), feature(const_trait_impl))]

use thiserror::Error;
use core::str::FromStr;

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
/// fn main() {
///     #[cfg(any(doc,test,doctest, feature = "const_trait_impl"))]
///     const res_kind: Result<PrimIntKind, PrimIntKindParsingError> = PrimIntKind::from_str("u16");
///     #[cfg(not(any(doc,test,doctest, feature = "const_trait_impl")))]
///     unimplemented!();
///     assert_eq!(res_kind, Ok(PrimIntKind::U16));
/// }
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

/// Error that is returned when parsing fails
/// 
/// # Example
/// 
/// ```
/// use prim_int_kind::{PrimIntKind, PrimIntKindParsingError};
/// 
/// // U16 is not a primitive integer kind due to case
/// let res_kind = "U16".parse::<PrimIntKind>();
/// assert_eq!(res_kind, Err(PrimIntKindParsingError));
/// ```
#[derive(Error, Debug, PartialEq)]
#[error("the provided &str is not that of a primitive integer")]
pub struct PrimIntKindParsingError;

#[cfg(any(doc,test,doctest, feature = "const_trait_impl"))]
macro_rules! impl_trait {
    (::$konst_crate:ident, $trait_name:ident, $t:ty, $macro_name:ident) => {
        impl const $trait_name for $t {
            $macro_name!($konst_crate);
        }
    }
}

#[cfg(not(any(doc,test,doctest, feature = "const_trait_impl")))]
macro_rules! impl_trait {
    (::$konst_crate:ident, $trait_name:ident, $t:ty, $macro_name:ident) => {
        impl $trait_name for $t {
            $macro_name!($konst_crate);
        }
    }
}

macro_rules! impl_from_str_for_prim_int_kind {
    ($konst_crate:ident) => {
        type Err = PrimIntKindParsingError;
        fn from_str(s: &str) -> Result<PrimIntKind, <Self as core::str::FromStr>::Err> {
            use ::$konst_crate::eq_str;
            // TODO: consider using Aho-Corasick algorithm
            match s {
                _ if eq_str(s, "u8") => Ok(PrimIntKind::U8),
                _ if eq_str(s, "u16") => Ok(PrimIntKind::U16),
                _ if eq_str(s, "u32") => Ok(PrimIntKind::U32),
                _ if eq_str(s, "u64") => Ok(PrimIntKind::U64),
                _ if eq_str(s, "u128") => Ok(PrimIntKind::U128),
                _ if eq_str(s, "usize") => Ok(PrimIntKind::Usize),
                _ if eq_str(s, "i8") => Ok(PrimIntKind::I8),
                _ if eq_str(s, "i16") => Ok(PrimIntKind::I16),
                _ if eq_str(s, "i32") => Ok(PrimIntKind::I32),
                _ if eq_str(s, "i64") => Ok(PrimIntKind::I64),
                _ if eq_str(s, "i128") => Ok(PrimIntKind::I128),
                _ if eq_str(s, "isize") => Ok(PrimIntKind::Isize),
                _ => Err(PrimIntKindParsingError),
            }
        }
    };
}

impl_trait!(::konst, FromStr, PrimIntKind, impl_from_str_for_prim_int_kind);

#[cfg(test)]
mod tests {
    use crate::{PrimIntKind, PrimIntKindParsingError};

    #[test]
    fn it_works() {
        let res_kind = "u16".parse();
        assert_eq!(res_kind, Ok(PrimIntKind::U16));
    }

    #[test]
    fn the_error_can_be_handled_easily() {
        // U16 is not a primitive integer kind due to case
        let res_kind = "U16".parse::<PrimIntKind>();
        assert_eq!(res_kind, Err(PrimIntKindParsingError));
    }
}
