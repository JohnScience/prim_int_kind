use core::str::FromStr;

use crate::PrimIntKind;

use thiserror::Error;

macro_rules! prim_int_kind_variant {
    (u8) => {
        U8
    };
    (u16) => {
        U16
    };
    (u32) => {
        U32
    };
    (u64) => {
        U64
    };
    (u128) => {
        U128
    };
    (usize) => {
        Usize
    };
    (i8) => {
        I8
    };
    (i16) => {
        I16
    };
    (i32) => {
        I32
    };
    (i64) => {
        I64
    };
    (i128) => {
        I128
    };
    (isize) => {
        Isize
    };
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

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_trait {
    (::$konst_crate:ident, $trait_name:ident, $t:ty, $macro_name:ident) => {
        impl const $trait_name for $t {
            $macro_name!($konst_crate);
        }
    };
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_trait {
    (::$konst_crate:ident, $trait_name:ident, $t:ty, $macro_name:ident) => {
        impl $trait_name for $t {
            $macro_name!($konst_crate);
        }
    };
}

macro_rules! impl_from_str_for_prim_int_kind {
    ($konst_crate:ident) => {
        type Err = PrimIntKindParsingError;
        fn from_str(s: &str) -> Result<PrimIntKind, <Self as core::str::FromStr>::Err> {
            use ::$konst_crate::eq_str;
            #[allow(unused_imports)]
            use PrimIntKind::*;
            // TODO: consider using Aho-Corasick algorithm
            impl_from_str_for_prim_int_kind!(
                @MATCH s for @PRIM_INTS with eq_str
            )
        }
    };
    (@MATCH $s:ident for @PRIM_INTS with $fn_name:ident) => {
        impl_from_str_for_prim_int_kind!(
            @MATCH $s for [u8,u16,u32,u64,u128,usize,i8,i16,i32,i64,i128,isize] with $fn_name
        )
    };
    (@MATCH $s:ident for [$($t:ident),+] with $fn_name:ident) => {
        match $s {
            $(
                _ if $fn_name($s, stringify!($t)) => Ok(prim_int_kind_variant!($t)),
            )+
            _ => Err(PrimIntKindParsingError),
        }
    };
}

impl_trait!(
    ::konst,
    FromStr,
    PrimIntKind,
    impl_from_str_for_prim_int_kind
);

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