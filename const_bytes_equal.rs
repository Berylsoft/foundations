pub const fn const_bytes_equal(lhs: &[u8], rhs: &[u8]) -> bool {
    if lhs.len() != rhs.len() {
        return false;
    }
    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }
    true
}

pub const fn const_str_equal(lhs: &str, rhs: &str) -> bool {
    const_bytes_equal(lhs.as_bytes(), rhs.as_bytes())
}
