#[inline]
pub fn u64_usize(n: u64) -> usize {
    n.try_into().expect("FATAL: u64 length to usize error")
}

#[inline]
pub fn usize_u64(n: usize) -> u64 {
    n.try_into().expect("FATAL: usize length to u64 error")
}

#[inline]
pub fn u32_usize(n: u32) -> usize {
    n.try_into().expect("FATAL: u32 length to usize error")
}
