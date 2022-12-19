#[macro_export]
macro_rules! num_enum {
    {
        $(#[$meta:meta])*
        $pub:vis enum $name:ident {
            $($variant:ident = $n:literal,)*
        } as $num:ident else $error:ident::$error_variant:ident
    } => {
        #[repr($num)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        $(#[$meta])*
        $pub enum $name {
            $($variant = $n,)*
        }

        impl TryFrom<$num> for $name {
            type Error = $error;
            
            fn try_from(n: $num) -> Result<$name, $error> {
                match n {
                    $($n => Ok($name::$variant),)*
                    n => Err($error::$error_variant(n)),
                }
            }
        }
    };
}
