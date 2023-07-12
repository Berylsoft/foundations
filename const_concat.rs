use core::mem::ManuallyDrop;

#[repr(C)]
struct Concat<const LA: usize, const LB: usize>([u8; LA], [u8; LB]);

#[repr(C)]
union ConcatCast<const LA: usize, const LB: usize, const LS: usize> {
    from: ManuallyDrop<Concat<LA, LB>>,
    to: ManuallyDrop<[u8; LS]>,
}

pub const unsafe fn const_concat_2_byte_arrays<const LA: usize, const LB: usize, const LS: usize>(a: [u8; LA], b: [u8; LB]) -> [u8; LS] {
    assert!(LA + LB == LS);
    ManuallyDrop::into_inner(ConcatCast { from: ManuallyDrop::new(Concat(a, b)) }.to)
}

#[macro_export]
macro_rules! const_concat {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        const LA: usize = $a.len();
        const LB: usize = $b.len();
        const LS: usize = $a.len() + $b.len();
        const A: [u8; LA] = unsafe { *$a.as_ptr().cast() };
        const B: [u8; LB] = unsafe { *$b.as_ptr().cast() };
        const S: [u8; LS] = unsafe { $crate::const_concat::const_concat_2_byte_arrays::<LA, LB, LS>(A, B) };
        const PS: &str = unsafe { core::mem::transmute(S.as_slice()) };
        PS
    }};
    ($a:expr, $($rest:expr),*) => {{
        const_concat!($a, const_concat!($($rest),*))
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat!($a, $($rest),*)
    };
}

#[macro_export]
macro_rules! const_concat_bytes {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        const LA: usize = $a.len();
        const LB: usize = $b.len();
        const LS: usize = $a.len() + $b.len();
        const A: [u8; LA] = unsafe { *$a.as_ptr().cast() };
        const B: [u8; LB] = unsafe { *$b.as_ptr().cast() };
        const S: [u8; LS] = unsafe { $crate::const_concat::const_concat_2_byte_arrays::<LA, LB, LS>(A, B) };
        const PS: &[u8] = S.as_slice();
        PS
    }};
    ($a:expr, $($rest:expr),*) => {{
        const_concat_bytes!($a, const_concat_bytes!($($rest),*))
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat_bytes!($a, $($rest),*)
    };
}

#[macro_export]
macro_rules! const_concat_byte_arrays {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        const LA: usize = $a.len();
        const LB: usize = $b.len();
        const LS: usize = $a.len() + $b.len();
        const A: [u8; LA] = $a;
        const B: [u8; LB] = $b;
        const S: [u8; LS] = unsafe { $crate::const_concat::const_concat_2_byte_arrays::<LA, LB, LS>(A, B) };
        S
    }};
    ($a:expr, $($rest:expr),*) => {{
        const_concat_byte_arrays!($a, const_concat_byte_arrays!($($rest),*))
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat_byte_arrays!($a, $($rest),*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_level_constants() {
        const SALUTATION: &str = "Hello";
        const TARGET: &str = "world";
        const GREETING: &str = const_concat!(SALUTATION, ", ", TARGET, "!");
        const GREETING_TRAILING_COMMA: &str = const_concat!(SALUTATION, ", ", TARGET, "!",);

        assert_eq!(GREETING, "Hello, world!");
        assert_eq!(GREETING_TRAILING_COMMA, "Hello, world!");
    }

    #[test]
    fn top_level_constants_bytes() {
        const SALUTATION: &[u8] = b"Hello";
        const TARGET: &[u8] = b"world";
        const GREETING: &[u8] = const_concat_bytes!(SALUTATION, b", ", TARGET, b"!");
        const GREETING_TRAILING_COMMA: &[u8] = const_concat_bytes!(SALUTATION, b", ", TARGET, b"!",);

        assert_eq!(GREETING, b"Hello, world!");
        assert_eq!(GREETING_TRAILING_COMMA, b"Hello, world!");
    }

    #[test]
    fn top_level_constants_byte_arrays() {
        const SALUTATION: [u8; 5] = *b"Hello";
        const TARGET: [u8; 5] = *b"world";
        const GREETING: [u8; 13] = const_concat_byte_arrays!(SALUTATION, *b", ", TARGET, *b"!");
        const GREETING_TRAILING_COMMA: [u8; 13] = const_concat_byte_arrays!(SALUTATION, *b", ", TARGET, *b"!",);

        assert_eq!(GREETING, *b"Hello, world!");
        assert_eq!(GREETING_TRAILING_COMMA, *b"Hello, world!");
    }
}
