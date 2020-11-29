use std::io::{self, Read, Write};

/// Auto trait for any type implementing
/// [Read](std::io::Read) + [Write](std::io::Write).
/// Sole purpose of this trait is to
/// make dyn [Read](std::io::Read) + [Write](std::io::Write) kind of possible.
pub trait StreamMarker: Read + Write {}
impl<T: Read + Write> StreamMarker for T {}

/// Adopted from `byteorder` crate.
pub(crate) trait ReadBytesExt: io::Read {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; std::mem::size_of::<u8>()];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    #[inline]
    fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0; std::mem::size_of::<i8>()];
        self.read_exact(&mut buf)?;
        Ok(buf[0] as i8)
    }

    #[inline]
    fn read_u16_be(&mut self) -> io::Result<u16> {
        let mut buf = [0; std::mem::size_of::<u16>()];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    #[inline]
    fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut buf = [0; std::mem::size_of::<u16>()];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    #[inline]
    fn read_u16_ne(&mut self) -> io::Result<u16> {
        let mut buf = [0; std::mem::size_of::<u16>()];
        self.read_exact(&mut buf)?;
        Ok(u16::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i16_be(&mut self) -> io::Result<i16> {
        let mut buf = [0; std::mem::size_of::<i16>()];
        self.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }

    #[inline]
    fn read_i16_le(&mut self) -> io::Result<i16> {
        let mut buf = [0; std::mem::size_of::<i16>()];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    #[inline]
    fn read_i16_ne(&mut self) -> io::Result<i16> {
        let mut buf = [0; std::mem::size_of::<i16>()];
        self.read_exact(&mut buf)?;
        Ok(i16::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u32_be(&mut self) -> io::Result<u32> {
        let mut buf = [0; std::mem::size_of::<u32>()];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    #[inline]
    fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0; std::mem::size_of::<u32>()];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    #[inline]
    fn read_u32_ne(&mut self) -> io::Result<u32> {
        let mut buf = [0; std::mem::size_of::<u32>()];
        self.read_exact(&mut buf)?;
        Ok(u32::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i32_be(&mut self) -> io::Result<i32> {
        let mut buf = [0; std::mem::size_of::<i32>()];
        self.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }

    #[inline]
    fn read_i32_le(&mut self) -> io::Result<i32> {
        let mut buf = [0; std::mem::size_of::<i32>()];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    #[inline]
    fn read_i32_ne(&mut self) -> io::Result<i32> {
        let mut buf = [0; std::mem::size_of::<i32>()];
        self.read_exact(&mut buf)?;
        Ok(i32::from_ne_bytes(buf))
    }

    #[inline]
    fn read_u64_be(&mut self) -> io::Result<u64> {
        let mut buf = [0; std::mem::size_of::<u64>()];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    #[inline]
    fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut buf = [0; std::mem::size_of::<u64>()];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    #[inline]
    fn read_u64_ne(&mut self) -> io::Result<u64> {
        let mut buf = [0; std::mem::size_of::<u64>()];
        self.read_exact(&mut buf)?;
        Ok(u64::from_ne_bytes(buf))
    }

    #[inline]
    fn read_i64_be(&mut self) -> io::Result<i64> {
        let mut buf = [0; std::mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    #[inline]
    fn read_i64_le(&mut self) -> io::Result<i64> {
        let mut buf = [0; std::mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    #[inline]
    fn read_i64_ne(&mut self) -> io::Result<i64> {
        let mut buf = [0; std::mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;
        Ok(i64::from_ne_bytes(buf))
    }
}

// TODO: Tests
impl<R: io::Read + ?Sized> ReadBytesExt for R {}

/// Adopted from `byteorder` crate.
pub trait WriteBytesExt: io::Write {
    #[inline]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n])
    }

    #[inline]
    fn write_i8(&mut self, n: i8) -> io::Result<()> {
        self.write_all(&[n as u8])
    }

    #[inline]
    fn write_u16_be(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_u16_le(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_u16_ne(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }

    #[inline]
    fn write_i16_be(&mut self, n: i16) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_i16_le(&mut self, n: i16) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_i16_ne(&mut self, n: i16) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }

    #[inline]
    fn write_u32_be(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_u32_le(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_u32_ne(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }

    #[inline]
    fn write_i32_be(&mut self, n: i32) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_i32_le(&mut self, n: i32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_i32_ne(&mut self, n: i32) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }

    #[inline]
    fn write_u64_be(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_u64_le(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_u64_ne(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }

    #[inline]
    fn write_i64_be(&mut self, n: i64) -> io::Result<()> {
        self.write_all(&n.to_be_bytes())
    }

    #[inline]
    fn write_i64_le(&mut self, n: i64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes())
    }

    #[inline]
    fn write_i64_ne(&mut self, n: i64) -> io::Result<()> {
        self.write_all(&n.to_ne_bytes())
    }
}

// TODO: Tests
impl<W: io::Write + ?Sized> WriteBytesExt for W {}
