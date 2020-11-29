use super::{pad, Serialize, BYTE_ORDER, PROTOCOL_MAJOR_VERSION, PROTOCOL_MINOR_VERSION};
use crate::utils::WriteBytesExt;
use std::convert::TryFrom;
use std::io::{self, Write};
use std::num::TryFromIntError;

/// Request a connection to X server
pub(crate) struct SetupRequest {
    auth_protocol_name: Vec<u8>,
    auth_protocol_data: Vec<u8>,
}

impl SetupRequest {
    pub fn new(
        auth_protocol_name: &[u8],
        auth_protocol_data: &[u8],
    ) -> Result<SetupRequest, TryFromIntError> {
        // Ensure protocol data and name len() fit to u16 required by protocol.
        u16::try_from(auth_protocol_name.len())?;
        u16::try_from(auth_protocol_data.len())?;

        let auth_protocol_name = auth_protocol_name.to_vec();
        let auth_protocol_data = auth_protocol_data.to_vec();

        Ok(SetupRequest {
            auth_protocol_name,
            auth_protocol_data,
        })
    }
}

// 1                       byte-order
//       #x42     MSB first
//       #x6C     LSB first
// 1                       unused
// 2     CARD16            protocol-major-version
// 2     CARD16            protocol-minor-version
// 2     n                 length of authorization-protocol-name
// 2     d                 length of authorization-protocol-data
// 2                       unused
// n     STRING8           authorization-protocol-name
// p                       unused, p=pad(n)
// d     STRING8           authorization-protocol-data
// q                       unused, q=pad(d)
impl Serialize for SetupRequest {
    fn serialize<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u8(BYTE_ORDER)?;
        writer.write_u8(0)?; // pad
        writer.write_u16_ne(PROTOCOL_MAJOR_VERSION)?;
        writer.write_u16_ne(PROTOCOL_MINOR_VERSION)?;
        // Safety: We ensure protocol name and data are u16 in new(),
        // and never allow build the struct any other way.
        writer.write_u16_ne(self.auth_protocol_name.len() as u16)?;
        writer.write_u16_ne(self.auth_protocol_data.len() as u16)?;
        writer.write_u8(0)?; // pad
        writer.write_u8(0)?; // pad

        writer.write_all(&self.auth_protocol_name[..])?;
        for _ in 0..pad(self.auth_protocol_name.len()) {
            writer.write_u8(0)?;
        }

        writer.write_all(&self.auth_protocol_data[..])?;
        for _ in 0..pad(self.auth_protocol_data.len()) {
            writer.write_u8(0)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SetupRequest;
    use crate::protocol::Serialize;
    use std::io::Cursor;

    #[test]
    fn test_serialize_empty() {
        const EXPECTED_AUTHORIZATION_BUF: &[u8] = b"l\0\x0b\0\0\0\0\0\0\0\0\0";
        let mut write_buf = vec![];

        SetupRequest::new(b"", b"")
            .expect("Empty vecs always pass")
            .serialize(&mut Cursor::new(&mut write_buf))
            .unwrap();

        assert_eq!(write_buf, EXPECTED_AUTHORIZATION_BUF);
    }

    #[test]
    fn test_serialize_auth_data() {
        const EXPECTED_AUTHORIZATION_BUF: &[u8] =
            b"l\0\x0b\0\0\0\t\0\t\0\0\0auth_name\0\0\0auth_data\0\0\0";
        let mut write_buf = vec![];

        SetupRequest::new(b"auth_name", b"auth_data")
            .expect("Specified values always pass")
            .serialize(&mut Cursor::new(&mut write_buf))
            .unwrap();

        assert_eq!(write_buf, EXPECTED_AUTHORIZATION_BUF);
    }
}
