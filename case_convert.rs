use alloc::string::String;

#[inline]
pub fn to_snake_case(s: &str) -> String {
    s.replace('-', "_")
}

// Copied from serde: serde_derive/src/internals/case.rs
pub fn to_pascal_case(s: &str) -> String {
    let mut pascal = String::new();
    let mut capitalize = true;
    for ch in s.chars() {
        if ch == '-' {
            capitalize = true;
        } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            pascal.push(ch);
        }
    }
    pascal
}
