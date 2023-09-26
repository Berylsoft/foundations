#[macro_export]
macro_rules! trailing_zero_byte {
    ($n:expr) => {{
        // after dividing by 8, trailing bytes in this amount must be all 0,
        // because the remainder is the trailing zero of last byte that is not 0.
        (8 - ($n.trailing_zeros() / 8)) as usize
    }};
}

#[macro_export]
macro_rules! leading_zero_byte {
    ($n:expr) => {{
        // after dividing by 8, leading bytes in this amount must be all 0,
        // because the remainder is the leading zero of last byte that is not 0.
        (8 - ($n.leading_zeros() / 8)) as usize
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(trailing_zero_byte!(0x00_00_00_00_00_00_00_00u64), 0);
        assert_eq!(trailing_zero_byte!(0x01_00_00_00_00_00_00_00u64), 1);
        assert_eq!(trailing_zero_byte!(0x01_00_00_01_00_00_00_00u64), 4);
        assert_eq!(trailing_zero_byte!(0x01_00_00_01_10_00_00_00u64), 5);
        assert_eq!(trailing_zero_byte!(0x01_00_00_01_00_00_00_01u64), 8);

        assert_eq!(leading_zero_byte!(0x00_00_00_00_00_00_00_00u64), 0);
        assert_eq!(leading_zero_byte!(0x00_00_00_00_00_00_00_01u64), 1);
        assert_eq!(leading_zero_byte!(0x00_00_00_00_01_00_00_01u64), 4);
        assert_eq!(leading_zero_byte!(0x00_00_00_10_01_00_00_01u64), 5);
        assert_eq!(leading_zero_byte!(0x01_00_00_00_01_00_00_01u64), 8);
    }
}
