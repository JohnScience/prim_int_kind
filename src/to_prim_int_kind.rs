use crate::PrimIntKind;

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

pub trait ToPrimIntKind {
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
    ToPrimIntKind::to_prim_int_kind() -> PrimIntKind
        for @PRIM_INTS
        as prim_int_kind_variant
);
