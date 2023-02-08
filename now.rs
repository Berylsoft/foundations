pub fn now_raw() -> (bool, u64, u32) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let res = SystemTime::now().duration_since(UNIX_EPOCH);
    let ok = res.is_ok();
    let dur = res.unwrap_or_else(|err| err.duration());
    (ok, dur.as_secs(), dur.subsec_nanos())
}
