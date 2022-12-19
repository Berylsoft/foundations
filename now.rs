#[inline]
pub fn now_raw() -> (u64, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).expect("FATAL: system time before unix epoch");
    (dur.as_secs(), dur.subsec_nanos())
}
