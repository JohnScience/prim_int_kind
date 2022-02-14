use crate::PrimIntKind;

macro_rules! prim_int_kind_variant {
    (u8) => {
        U8(Default::default())
    };
    (u16) => {
        U16(Default::default())
    };
    (u32) => {
        U32(Default::default())
    };
    (u64) => {
        U64(Default::default())
    };
    (u128) => {
        U128(Default::default())
    };
    (usize) => {
        Usize(Default::default())
    };
    (i8) => {
        I8(Default::default())
    };
    (i16) => {
        I16(Default::default())
    };
    (i32) => {
        I32(Default::default())
    };
    (i64) => {
        I64(Default::default())
    };
    (i128) => {
        I128(Default::default())
    };
    (isize) => {
        Isize(Default::default())
    };
}

/// Extension trait that for all primitive integers implements an associated function
/// that returns a [`PrimIntKind`] representing a particular primitive integer.
pub trait ToPrimIntKindExt {
    fn to_prim_int_kind() -> PrimIntKind;
}

macro_rules! impl_trait {
    (
        $trait_name:ident::$fn_name:ident() -> $prim_int_kind:ident
            for @PRIM_INTS
            as $prim_int_kind_variant:ident
    ) => {
        impl_trait!(
            $trait_name::$fn_name() -> $prim_int_kind
                for [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]
                as $prim_int_kind_variant
        );
    };
    (
        $trait_name:ident::$fn_name:ident() -> $prim_int_kind:ident
            for [$($t:ident),+]
            as $prim_int_kind_variant:ident
    ) => {
        $(
            impl $trait_name for $t {
                fn $fn_name() -> $prim_int_kind {
                    use $prim_int_kind::*;
                    prim_int_kind_variant!($t)
                }
            }
        )+
    };
}

impl_trait!(
    ToPrimIntKindExt::to_prim_int_kind() -> PrimIntKind
        for @PRIM_INTS
        as prim_int_kind_variant
);
