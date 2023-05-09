#[macro_export]
macro_rules! const_assert {
    ($b:expr) => {
        const _: [(); !$b as usize] = [];
    };
}

#[macro_export]
macro_rules! const_assert_eq {
    ($a:expr, $b:expr) => {
        const _: [(); !{ $a == $b } as usize] = [];
    };
}
