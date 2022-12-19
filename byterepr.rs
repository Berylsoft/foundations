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
        self.clone()
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

#[macro_export]
macro_rules! byterepr_struct_impl {
    {
        $struct_name:ident {
            $($field:ident: $ty:ty,)*
        }
    } => {
        impl ByteRepr for $struct_name {
            const SIZE: usize = { $(<$ty as ByteRepr>::SIZE + )* 0 };
            type Bytes = [u8; Self::SIZE];

            fn from_bytes(bytes: Self::Bytes) -> Self {
                let mut offset: usize = 0;
                $(let $field = {
                    let size = <$ty as ByteRepr>::SIZE;
                    let slice = bytes[offset..(offset + size)].try_into().unwrap();
                    let val = <$ty as ByteRepr>::from_bytes(slice);
                    offset += size;
                    val
                };)*
                assert_eq!(offset, Self::SIZE);
                Self { $($field,)* }
            }

            fn to_bytes(&self) -> Self::Bytes {
                let mut bytes = [0u8; Self::SIZE];
                let mut offset: usize = 0;
                $({
                    let size = <$ty as ByteRepr>::SIZE;
                    let slice = ByteRepr::to_bytes(&self.$field);
                    (&mut bytes[offset..(offset + size)]).copy_from_slice(slice.as_ref());
                    offset += size;
                })*
                assert_eq!(offset, Self::SIZE);
                bytes
            }
        }
    };
}

#[macro_export]
macro_rules! byterepr_struct {
    {
        $(#[$meta:meta])*
        $pub:vis struct $struct_name:ident {
            $(#[$field_meta:meta])*
            $($field_pub:vis $field:ident: $ty:ty,)*
        }
    } => {
        $(#[$meta])*
        $pub struct $struct_name {
            $(#[$field_meta])*
            $($field_pub $field: $ty,)*
        }

        byterepr::byterepr_struct_impl! {
            $struct_name {
                $($field: $ty,)*
            }
        }
    };
}
