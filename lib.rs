#![deny(unused_results)]

#![no_std]
#[cfg(feature = "alloc")] extern crate alloc;
#[cfg(feature = "std")] extern crate std;

macro_rules! decl_modules {
    ($($pub:vis $name:ident $feature_name:literal)*) => {$(
        #[cfg(feature = $feature_name)]
        pub mod $name;
    )*};
}

decl_modules! {
    // common modules
    pub xor "xor"
    pub now "now"
    pub timestamp "timestamp"
    pub usize_casting "usize-casting"
    pub map_util "map-util"
    pub byterepr "byterepr"
    pub bytes_read "bytes-read"
    pub num_compress "num-compress"
    pub case_convert "case-convert"
    pub const_bytes_equal "const-bytes-equal"
    pub key_index "key-index"
    pub key_index_map "key-index-map"
    pub fs "fs"

    // pure macros
    num_compress_macros "num-compress-macros"
    concat_string "concat-string" 
    vec_ext "vec-ext"
    error_enum "error-enum"
    num_enum "num-enum"
    byterepr_macros "byterepr-macros"

    // macros with helper functions
    pub const_concat "const-concat"
}
