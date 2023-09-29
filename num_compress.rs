macro_rules! zigzag_impl {
    // TODO auto make fn name with concat_ident! and const case convert
    ($($enc_fn_name:ident $dec_fn_name:ident $ity:ident $uty:ty)*) => {$(
        #[inline]
        pub const fn $enc_fn_name(n: $ity) -> $uty {
            // TODO <$uty>::BITS returns u32 whatever $uty is. Does it matter?
            ((n << 1) ^ (n >> (<$uty>::BITS - 1))) as $uty
        }

        #[inline]
        pub const fn $dec_fn_name(n: $uty) -> $ity {
            ((n >> 1) ^ (-((n & 1) as $ity)) as $uty) as $ity
        }
    )*};
}

zigzag_impl! {
    zigzag_encode_i8 zigzag_decode_i8 i8 u8
    zigzag_encode_i16 zigzag_decode_i16 i16 u16
    zigzag_encode_i32 zigzag_decode_i32 i32 u32
    zigzag_encode_i64 zigzag_decode_i64 i64 u64
}

// TODO tests
