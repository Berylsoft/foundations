pub use tiny_keccak::{Shake, Sha3, Hasher, Xof};

pub fn shake256_once<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut hasher = Shake::v256();
    hasher.update(bytes);
    let mut res = [0; N];
    hasher.squeeze(&mut res);
    res
}

pub fn sha3_256_once(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3::v256();
    hasher.update(bytes);
    let mut res = [0; 32];
    hasher.finalize(&mut res);
    res
}

pub fn sha3_512_once(bytes: &[u8]) -> [u8; 64] {
    let mut hasher = Sha3::v512();
    hasher.update(bytes);
    let mut res = [0; 64];
    hasher.finalize(&mut res);
    res
}
