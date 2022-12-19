#[macro_export]
macro_rules! error_enum {
    {
        $(#[$meta:meta])*
        $pub:vis enum $name:ident {
            $($extra:tt)*
        }
        convert {
            $($variant:ident => $error:ty,)*
        }
    } => {
        $(#[$meta])*
        $pub enum $name {
            $($variant($error),)*
            $($extra)*
        }

        $(
            impl From<$error> for $name {
                fn from(err: $error) -> $name {
                    <$name>::$variant(err)
                }
            }
        )*
    };
}
