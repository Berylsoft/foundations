// deprecated by byterepr
// mod bin_struct_deprecated;

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

// Copied from https://github.com/FaultyRAM/concat-string@1.0.1
// Copyright (c) 2017-2018 FaultyRAM. Licensed under the Apache License, Version 2.0 or the MIT license.
#[macro_export]
macro_rules! concat_string {
    () => { String::with_capacity(0) };
    ($($s:expr),+) => {{
        use std::ops::AddAssign;
        let mut len = 0;
        $(len.add_assign(AsRef::<str>::as_ref(&$s).len());)+
        let mut buf = String::with_capacity(len);
        $(buf.push_str($s.as_ref());)+
        buf
    }};
}
