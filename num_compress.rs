#[inline]
pub const fn zigzag_encode(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[inline]
pub const fn zigzag_decode(n: u64) -> i64 {
    ((n >> 1) ^ (-((n & 1) as i64)) as u64) as i64
}

#[inline]
pub fn float_find_zero(f: u64) -> usize {
    // after dividing by 8, trailing bytes in this amount must be all 0,
    // because the remainder is the trailing zero of last byte that is not 0.
    (8 - (f.trailing_zeros() / 8)) as usize
}

#[inline]
pub const fn from_h4l4(h4: u8, l4: u8) -> u8 {
    assert!(h4 <= 0xF);
    assert!(l4 <= 0xF);
    h4 << 4 | l4
}

#[inline]
pub const fn to_h4l4(n: u8) -> (u8, u8) {
    (n >> 4, n & 0xF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(float_find_zero(0x00_00_00_00_00_00_00_00), 0);
        assert_eq!(float_find_zero(0x01_00_00_00_00_00_00_00), 1);
        assert_eq!(float_find_zero(0x01_00_00_01_00_00_00_00), 4);
        assert_eq!(float_find_zero(0x01_00_00_01_10_00_00_00), 5);
        assert_eq!(float_find_zero(0x01_00_00_01_00_00_00_01), 8);
    }
}
