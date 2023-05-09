pub fn xor(dst: &mut [u8], src: &[u8]) {
    let len = dst.len();

    // Check *before* looping that both are long enough,
    // in a way that makes it directly obvious to LLVM
    // that the indexing below will be in-bounds.
    // ref: https://users.rust-lang.org/t/93119/10
    let (dst, src) = (&mut dst[..len], &src[..len]);

    for i in 0..len {
        dst[i] ^= src[i];
    }
}

pub fn xor_array<const N: usize>(dst: &mut [u8; N], src: &[u8; N]) {
    for i in 0..N {
        dst[i] ^= src[i];
    }
}
