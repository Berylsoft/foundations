pub trait ByteRepr {
    const SIZE: usize;
    type Bytes: AsRef<[u8]>;
    // #![feature(associated_type_defaults)]
    // type Bytes: AsRef<[u8]> = [u8; Self::SIZE];

    fn from_bytes(bytes: Self::Bytes) -> Self;
    fn to_bytes(&self) -> Self::Bytes;
}

impl<const N: usize> ByteRepr for [u8; N] {
    const SIZE: usize = N;
    type Bytes = [u8; N];

    #[inline]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        bytes
    }

    #[inline]
    fn to_bytes(&self) -> Self::Bytes {
        *self
    }
}

macro_rules! byterepr_num_impl {
    ($($ty:ty)*) => {$(
        impl ByteRepr for $ty {
            const SIZE: usize = core::mem::size_of::<Self>();
            type Bytes = [u8; Self::SIZE];
            
            #[inline]
            fn from_bytes(bytes: Self::Bytes) -> Self {
                Self::from_be_bytes(bytes)
            }

            #[inline]
            fn to_bytes(&self) -> Self::Bytes {
                self.to_be_bytes()
            }
        }
    )*};
}

// should not implement for usize and isize
byterepr_num_impl!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
