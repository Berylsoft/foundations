pub const BS_IDENT: u32 = 0x42650000;

use std::{io::{self, Read, Write}};
use blake3::{Hasher, OUT_LEN as HASH_LEN};
use crate::usize_casting::*;

// region: util

fn usize_u32(n: usize) -> Result<u32> {
    n.try_into().map_err(|_| Error::TooLongSize { size: usize_u64(n) })
}

// endregion

// region: helper traits

macro_rules! read_sized_impl {
    ($self:expr, $len:expr) => {{
        let mut buf = [0; $len];
        $self.read_exact(&mut buf)?;
        Ok(buf)
    }};
}

trait ReadHelper: Read {
    #[inline]
    fn read_bytes(&mut self, len: usize) -> Result<Box<[u8]>> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }

    #[inline]
    fn read_u32(&mut self) -> Result<u32> {
        read_sized_impl!(self, 4).map(u32::from_be_bytes)
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        read_sized_impl!(self, 1).map(u8::from_be_bytes)
    }

    #[inline]
    fn read_hash(&mut self) -> Result<Hash> {
        read_sized_impl!(self, HASH_LEN)
    }
}

impl<R: Read> ReadHelper for R {}

trait WriteHelper: Write {
    #[inline]
    fn write_bytes<B: AsRef<[u8]>>(&mut self, bytes: B) -> io::Result<()> {
        self.write_all(bytes.as_ref())?;
        Ok(())
    }

    #[inline]
    fn write_u32(&mut self, val: u32) -> io::Result<()> {
        self.write_all(&val.to_be_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_u8(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&val.to_be_bytes())?;
        Ok(())
    }
}

impl<W: Write> WriteHelper for W {}

// endregion

// region: row types

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct KV {
    pub scope: Box<[u8]>,
    pub key: Box<[u8]>,
    pub value: Box<[u8]>,
}

pub type Hash = [u8; HASH_LEN];

const ROW_KV: u8 = 0;
const ROW_HASH: u8 = 1;
const ROW_END: u8 = 2;

#[derive(Debug, Clone)]
pub enum Row {
    KV(KV),
    Hash(Hash),
    End,
}

// endregion

// region: config types

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Sizes {
    pub scope: Option<u32>,
    pub key: Option<u32>,
    pub value: Option<u32>,
}

impl Sizes {
    fn flag(&self) -> u8 {
        let mut flag = 0;
        macro_rules! skv_op_impl {
            ($($x:ident,)*) => {$(
                if self.$x.is_some() {
                    flag |= SIZES_FLAG_BASES.$x;
                }
            )*};
        }
        skv_op_impl!(scope, key, value,);
        flag
    }
}

struct SizeFlagBases {
    scope: u8,
    key: u8,
    value: u8,
}

const SIZES_FLAG_BASES: SizeFlagBases = SizeFlagBases {
    scope: 1 << 0,
    key: 1 << 1,
    value: 1 << 2,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Config {
    pub ident: Box<[u8]>,
    pub sizes: Sizes,
}

// endregion

// region: error types

#[derive(Debug)]
pub enum InputKind {
    Scope,
    Key,
    Value,
}

impl<'a> From<&'a str> for InputKind {
    fn from(s: &'a str) -> Self {
        match s {
            "scope" => InputKind::Scope,
            "key" => InputKind::Key,
            "value" => InputKind::Value,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    VersionNotMatch { existing: u32 },
    ConfigNotMatch { existing: Config, current: Config },
    HashNotMatch { existing: Hash, calculated: Hash },
    InputLengthNotMatch { config_len: u32, input_len: u32, which: InputKind },
    UnexpectedRowType { row_type: u8 },
    TooLongSize { size: u64 },
    /// may happens only when using async-kvdump
    AsyncFileClosed,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// endregion

// region: reader

pub struct Reader<F: Read> {
    inner: F,
    config: Config,
    hasher: Hasher,
}

impl<F: Read> Reader<F> {
    #[inline]
    pub fn config(&self) -> &Config {
        &self.config
    }

    fn read_init(inner: &mut F) -> Result<Config> {
        let version = inner.read_u32()?;
        if version != BS_IDENT {
            return Err(Error::VersionNotMatch { existing: version });
        }

        let ident_len = u32_usize(inner.read_u32()?);
        let ident = inner.read_bytes(ident_len)?;

        let sizes_flag = inner.read_u8()?;
        macro_rules! skv_op_impl {
            ($($x:ident,)*) => {$(
                let $x = ((sizes_flag & SIZES_FLAG_BASES.$x) != 0).then_some(inner.read_u32()?);
            )*};
        }
        skv_op_impl!(scope, key, value,);
        let sizes = Sizes { scope, key, value };

        Ok(Config { ident, sizes })
    }

    pub fn read(&mut self) -> Result<Row> {
        Ok(match self.inner.read_u8()? {
            ROW_KV => Row::KV({
                macro_rules! skv_op_impl {
                    ($($x:ident,)*) => {$(
                        let len = u32_usize(match self.config.sizes.$x {
                            Some(len) => len,
                            None => self.inner.read_u32()?,
                        });
                        let $x = self.inner.read_bytes(len)?;
                        self.hasher.update(&$x);
                    )*};
                }
                skv_op_impl!(scope, key, value,);
                KV { scope, key, value }
            }),
            ROW_HASH => Row::Hash({
                let existing = self.inner.read_hash()?;
                let calculated = *self.hasher.finalize().as_bytes();
                if existing != calculated {
                    return Err(Error::HashNotMatch {
                        existing,
                        calculated,
                    });
                }
                self.hasher.reset();
                calculated
            }),
            ROW_END => Row::End,
            row_type => return Err(Error::UnexpectedRowType { row_type }),
        })
    }

    pub fn init(mut inner: F) -> Result<Reader<F>> {
        let config = Reader::read_init(&mut inner)?;
        Ok(Reader { inner, config, hasher: Hasher::new() })
    }
}

impl<F: Read> Iterator for Reader<F> {
    type Item = Result<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            Ok(Row::End) => None,
            result => Some(result),
        }
    }
}

// endregion

// region: writer

pub struct Writer<F: Write> {
    inner: F,
    config: Config,
    hasher: Hasher,
}

impl<F: Write> Writer<F> {
    #[inline]
    pub fn config(&self) -> &Config {
        &self.config
    }

    fn write_init(&mut self) -> Result<()> {
        self.inner.write_u32(BS_IDENT)?;

        self.inner.write_u32(usize_u32(self.config.ident.len())?)?;
        self.inner.write_bytes(self.config.ident.clone())?;

        self.inner.write_u8(self.config.sizes.flag())?;
        macro_rules! skv_op_impl {
            ($($x:ident,)*) => {$(
                self.inner.write_u32(self.config.sizes.$x.unwrap_or(0))?;
            )*};
        }
        skv_op_impl!(scope, key, value,);

        self.inner.flush()?;
        Ok(())
    }

    pub fn write_kv(&mut self, kv: KV) -> Result<()> {
        self.inner.write_u8(ROW_KV)?;

        macro_rules! skv_op_impl {
            ($($x:ident,)*) => {$({
                let input_len = usize_u32(kv.$x.len())?;
                match self.config.sizes.$x {
                    Some(config_len) => {
                        if config_len != input_len {
                            return Err(Error::InputLengthNotMatch {
                                config_len,
                                input_len,
                                which: stringify!($x).into(),
                            });
                        }
                    },
                    None => self.inner.write_u32(input_len)?,
                }
                self.hasher.update(&kv.$x);
                self.inner.write_bytes(kv.$x)?;
            })*};
        }
        skv_op_impl!(scope, key, value,);

        // TODO may too frequent
        self.inner.flush()?;
        Ok(())
    }

    pub fn write_hash(&mut self) -> Result<Hash> {
        self.inner.write_u8(ROW_HASH)?;

        let hash = *self.hasher.finalize().as_bytes();
        self.inner.write_bytes(&hash)?;

        self.inner.flush()?;
        Ok(hash)
    }

    fn write_end(&mut self) -> Result<()> {
        self.inner.write_u8(ROW_END)?;

        self.inner.flush()?;
        Ok(())
    }

    pub fn init(inner: F, config: Config) -> Result<Writer<F>> {
        let mut _self = Writer { inner, config, hasher: Hasher::new() };
        _self.write_init()?;
        Ok(_self)
    }
}

impl<F: Write> Drop for Writer<F> {
    fn drop(&mut self) {
        self.write_hash().expect("FATAL: Error occurred during writing final hash");
        self.write_end().expect("FATAL: Error occurred during writing end");
    }
}

// endregion
