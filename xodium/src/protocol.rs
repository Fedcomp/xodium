mod setup_codec;
mod setup_request;

pub(crate) use self::setup_codec::SetupCodec;

use std::io::{self, Read, Write};

#[cfg(target_endian = "big")]
/// Protocol byte order.
/// X Window protocol allows us to specify connection endianness,
/// and we use native endianness for compilation target platform.
pub const BYTE_ORDER: u8 = b'B';
#[cfg(target_endian = "little")]
/// Protocol byte order.
/// X Window protocol allows us to specify connection endianness,
/// and we use native endianness for compilation target platform.
pub const BYTE_ORDER: u8 = b'l';

/// X Window System protocol major version
pub(crate) const PROTOCOL_MAJOR_VERSION: u16 = 11;
/// X Window System protocol minor version
pub(crate) const PROTOCOL_MINOR_VERSION: u16 = 0;

#[allow(dead_code)]
pub(crate) type CARD8 = u8;
#[allow(dead_code)]
pub(crate) type CARD16 = u16;
#[allow(dead_code)]
pub(crate) type CARD32 = u32;
#[allow(dead_code)]
pub(crate) type CARD64 = u64;
#[allow(dead_code)]
pub(crate) type INT8 = i8;
#[allow(dead_code)]
pub(crate) type INT32 = i32;
#[allow(dead_code)]
pub(crate) type INT16 = i16;
#[allow(dead_code)]
pub(crate) type INT64 = i64;
#[allow(dead_code)]
pub(crate) type BYTE = u8;
#[allow(dead_code)]
pub(crate) type BOOL = bool;

/// General crate serialization trait.
pub trait Serialize {
    fn serialize<W: Write>(&self, writer: W) -> io::Result<()>;
}

/// General crate deserialization trait.
pub trait Deserialize: Sized {
    fn deserialize<R: Read>(reader: R) -> io::Result<Option<Self>>;
}

pub fn pad(e: usize) -> usize {
    (4 - (e % 4)) % 4
}
