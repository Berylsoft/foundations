#[inline]
const fn u64_i64(u: u64, s: bool) -> i64 {
    let i = u as i64;
    if s { i } else { -i }
}

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const MILLIS_PER_SEC: i64 = 1_000;

pub const AFTER_UNIX: i64 = 978307200;

pub const fn from_now_raw(dir: bool, secs: u64, nanos: u32) -> (i64, u32) {
    let secs = u64_i64(secs, dir) - AFTER_UNIX;
    (secs, nanos)
}

pub const fn from_unix_ms(ts: i64) -> (i64, u32) {
    let secs = ts / MILLIS_PER_SEC - AFTER_UNIX;
    let nanos = ((ts % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI;
    (secs, nanos)
}

pub const fn to_unix_ms(secs: i64, nanos: u32) -> i64 {
    (secs + AFTER_UNIX) * MILLIS_PER_SEC
    + (nanos / NANOS_PER_MILLI) as i64
}

pub fn from_nanos(nanos: i128) -> (i64, u32) {
    let secs = (nanos / (NANOS_PER_SEC as i128)) as i64;
    let nanos = (nanos % (NANOS_PER_SEC as i128)) as u32;
    (secs, nanos)
}

pub fn to_nanos(secs: i64, nanos: u32) -> i128 {
    (secs as i128) * (NANOS_PER_SEC as i128)
    + (nanos as i128)
}
