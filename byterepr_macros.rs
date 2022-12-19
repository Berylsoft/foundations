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

        $crate::byterepr_struct_impl! {
            $struct_name {
                $($field: $ty,)*
            }
        }
    };
}
