pub fn shake256_once<const N: usize>(bytes: &[u8]) -> [u8; N] {
    use tiny_keccak::{Shake, Hasher, Xof};
    let mut hasher = Shake::v256();
    hasher.update(bytes);
    let mut res = [0; N];
    hasher.squeeze(&mut res);
    res
}
