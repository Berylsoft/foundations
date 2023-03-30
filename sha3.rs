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

pub struct Shake256Cipher {
    hasher: Shake,
}

impl Shake256Cipher {
    pub fn init(key: &[u8]) -> Shake256Cipher {
        let mut hasher = Shake::v256();
        hasher.update(key);
        Shake256Cipher { hasher }
    }

    pub fn next(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut output = vec![0; bytes.len()];
        self.hasher.squeeze(&mut output);
        for i in 0..bytes.len() {
            output[i] ^= bytes[i];
        }
        output
    }
}
