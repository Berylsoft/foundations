// Copied from https://github.com/FaultyRAM/concat-string@1.0.1
// Copyright (c) 2017-2018 FaultyRAM. Licensed under the Apache License, Version 2.0 or the MIT license.
#[macro_export]
macro_rules! concat_string {
    () => { String::with_capacity(0) };
    ($($s:expr),+) => {{
        use core::ops::AddAssign;
        let mut len = 0;
        $(len.add_assign(AsRef::<str>::as_ref(&$s).len());)+
        let mut buf = String::with_capacity(len);
        $(buf.push_str($s.as_ref());)+
        buf
    }};
}
