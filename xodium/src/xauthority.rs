mod connection_family;

use self::connection_family::ConnectionFamily;
use crate::utils::ReadBytesExt;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{self, Read};
use std::path::PathBuf;

const DEFAULT_XAUTHORITY_FILE_NAME: &str = ".Xauthority";

/// Single entry from Xauthority file
#[derive(Debug, PartialEq)]
pub(crate) struct XAuthEntry {
    pub connection_family: ConnectionFamily,
    pub display_name: String,
    pub display_number: u16,
    pub protocol_name: String,
    pub protocol_data: Vec<u8>,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    Io(io::Error),
    InvalidFile,
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "Failed to process Xauthority: {}", e),
            ParseError::InvalidFile => write!(f, "invalid Xauthority file format"),
        }
    }
}

fn read_sized_string<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let len = reader.read_u16_be()?;
    let mut string = vec![];
    reader.take(u64::from(len)).read_to_end(&mut string)?;
    Ok(string)
}

fn read_entry<R: Read>(mut reader: R) -> Result<Option<XAuthEntry>, ParseError> {
    let raw_connection_family = match reader.read_u16_be() {
        Ok(v) => v,
        Err(e) if e.kind() == UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let connection_family =
        ConnectionFamily::try_from(raw_connection_family).ok_or(ParseError::InvalidFile)?;

    let raw_display_name = read_sized_string(&mut reader)?;
    let raw_display_number = read_sized_string(&mut reader)?;
    let raw_protocol_name = read_sized_string(&mut reader)?;
    let protocol_data = read_sized_string(&mut reader)?;

    let display_name = String::from_utf8_lossy(&raw_display_name).to_string();
    let display_number: u16 = String::from_utf8_lossy(&raw_display_number).parse().map_err(|_| ParseError::InvalidFile)?;
    let protocol_name = String::from_utf8_lossy(&raw_protocol_name).to_string();

    Ok(Some(XAuthEntry {
        connection_family,
        display_name,
        display_number,
        protocol_name,
        protocol_data,
    }))
}

/// Parse Xauthority contents from slice
pub(crate) fn from_reader<R: Read>(mut reader: R) -> Result<Vec<XAuthEntry>, ParseError> {
    let mut entries = vec![];

    loop {
        match read_entry(&mut reader) {
            Ok(Some(entry)) => entries.push(entry),
            Ok(None) => break,
            Err(e) => return Err(e),
        }
    }

    Ok(entries)
}

// TODO: Better error type
/// Try to parse xauth like normal x11 programm.
/// By default, will parse the file specified by the XAUTHORITY environment
/// variable or .Xauthority in the user's home directory.
pub(crate) fn read_default() -> Result<Vec<XAuthEntry>, ParseError> {
    // Either $XAUTHORITY or $HOME/.Xauthority
    let xauthority_path: PathBuf = match env::var_os("XAUTHORITY") {
        Some(p) => p.into(),
        None => match env::var_os("HOME") {
            Some(p) => PathBuf::from(p).join(DEFAULT_XAUTHORITY_FILE_NAME),
            None => {
                return Err(
                    io::Error::new(io::ErrorKind::NotFound, "No such file or directory").into(),
                )
            }
        },
    };

    Ok(from_reader(File::open(xauthority_path)?)?)
}

#[cfg(test)]
mod tests {
    use super::{from_reader, read_default, ConnectionFamily, XAuthEntry};
    use std::env;
    use std::fs::{create_dir_all, File};
    use std::io::{self, Write};

    const XAUTH_FILE_MULTIPLE_ENTRIES_EXAMPLE: &[u8] =
        b"\x01\0\0\x08hostname\0\x010\0\x12MIT-MAGIC-COOKIE-1\0\x03\xab\xcd\xef\x01\0\0\x08hostname\0\x011\0\x12MIT-MAGIC-COOKIE-1\0\x03\xab\xcd\xef";
    const XAUTH_FILE_SINGLE_ENTRY_EXAMPLE: &[u8] =
        b"\x01\0\0\x08hostname\0\x010\0\x12MIT-MAGIC-COOKIE-1\0\x03\xab\xcd\xef";

    #[test]
    fn test_read_from_string_single_entry() {
        assert_eq!(
            from_reader(XAUTH_FILE_SINGLE_ENTRY_EXAMPLE).unwrap()[0],
            XAuthEntry {
                connection_family: ConnectionFamily::Local,
                display_name: "hostname".into(),
                display_number: 0,
                protocol_name: "MIT-MAGIC-COOKIE-1".into(),
                protocol_data: b"\xAB\xCD\xEF".to_vec()
            }
        );
    }

    #[test]
    fn test_read_from_string_multiple_entries() {
        assert_eq!(
            from_reader(XAUTH_FILE_MULTIPLE_ENTRIES_EXAMPLE).unwrap(),
            vec![
                XAuthEntry {
                    connection_family: ConnectionFamily::Local,
                    display_name: "hostname".into(),
                    display_number: 0,
                    protocol_name: "MIT-MAGIC-COOKIE-1".into(),
                    protocol_data: b"\xAB\xCD\xEF".to_vec()
                },
                XAuthEntry {
                    connection_family: ConnectionFamily::Local,
                    display_name: "hostname".into(),
                    display_number: 1,
                    protocol_name: "MIT-MAGIC-COOKIE-1".into(),
                    protocol_data: b"\xAB\xCD\xEF".to_vec()
                }
            ]
        );
    }

    #[test]
    fn test_read_from_empty_string() {
        assert_eq!(from_reader(b"".as_ref()).unwrap(), vec![]);
    }

    #[test]
    fn test_read_from_string_eof() {
        assert!(from_reader(b"\x01\0\0\x08hostname".as_ref()).is_err());
    }

    #[test]
    fn test_read_default() -> io::Result<()> {
        let xauth_path = env::temp_dir().join("xodium-tests-xauth");
        let mut file = File::create(&xauth_path)?;
        file.write_all(XAUTH_FILE_SINGLE_ENTRY_EXAMPLE)?;

        let old_xauth = env::var_os("XAUTHORITY");
        env::set_var("XAUTHORITY", xauth_path);

        let result = read_default();

        if let Some(v) = old_xauth {
            env::set_var("XAUTHORITY", v);
        }

        assert_eq!(
            result.unwrap()[0],
            XAuthEntry {
                connection_family: ConnectionFamily::Local,
                display_name: "hostname".into(),
                display_number: 0,
                protocol_name: "MIT-MAGIC-COOKIE-1".into(),
                protocol_data: b"\xAB\xCD\xEF".to_vec()
            }
        );

        Ok(())
    }

    #[test]
    fn test_read_default_no_xauthority_env() -> io::Result<()> {
        let tmp_dir = env::temp_dir().join("xodium-tests-xauth-dir");
        let xauth_path = tmp_dir.join(".Xauthority");
        create_dir_all(&tmp_dir)?;
        let mut file = File::create(&xauth_path)?;
        file.write_all(XAUTH_FILE_SINGLE_ENTRY_EXAMPLE)?;

        let old_xauth = env::var_os("XAUTHORITY");
        let old_home = env::var_os("HOME");
        env::remove_var("XAUTHORITY");
        env::set_var("HOME", tmp_dir);

        let result = read_default();

        if let Some(v) = old_xauth {
            env::set_var("XAUTHORITY", v);
        }

        if let Some(v) = old_home {
            env::set_var("HOME", v);
        }

        assert_eq!(
            result.unwrap()[0],
            XAuthEntry {
                connection_family: ConnectionFamily::Local,
                display_name: "hostname".into(),
                display_number: 0,
                protocol_name: "MIT-MAGIC-COOKIE-1".into(),
                protocol_data: b"\xAB\xCD\xEF".to_vec()
            }
        );

        Ok(())
    }
}
