use crate::display::{Display, DisplayError};
use std::fmt;
use std::io;
use std::io::{Read, Write};
// TODO: Support other platforms in distant future?
use std::os::unix::net::UnixStream;

/// Xodium socket connection error
#[derive(Debug)]
pub enum ConnectionError {
    DisplayNotAvailable(DisplayError),
    Io(io::Error),
}

impl From<DisplayError> for ConnectionError {
    fn from(e: DisplayError) -> Self {
        ConnectionError::DisplayNotAvailable(e)
    }
}

impl From<io::Error> for ConnectionError {
    fn from(e: io::Error) -> Self {
        ConnectionError::Io(e)
    }
}

impl From<ConnectionError> for io::Error {
    fn from(e: ConnectionError) -> io::Error {
        match e {
            ConnectionError::Io(io) => io,
            ConnectionError::DisplayNotAvailable(disp) => {
                io::Error::new(io::ErrorKind::Other, disp.to_string())
            }
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionError::DisplayNotAvailable(e) => {
                write!(f, "X Connection failed: {}", e)
            }
            ConnectionError::Io(e) => {
                write!(f, "X Connection failed: {}", e)
            }
        }
    }
}

/// Connect to X server using DISPLAY environment variable or default value.
/// Function will first try to parse DISPLAY variable. In case it fails,
/// function will try to connect to :0 display (default for most environments)
pub fn connect_default() -> Result<Connection, ConnectionError> {
    let env_display = Display::from_env()?;
    connect_to_display(env_display)
}

const DEFAULT_UNIX_X_SERVER_SOCKET_PATH: &str = "/tmp/.X11-unix/X";

/// Connect to your specified address using [Display]
pub fn connect_to_display(display: Display) -> Result<Connection, ConnectionError> {
    if display.hostname.is_some() {
        unimplemented!("hostname connections are not supported at the moment");
    }

    if display.screen.is_some() {
        unimplemented!("screen connections are not supported at the moment");
    }

    let connection = UnixStream::connect(format!(
        "{}{}",
        DEFAULT_UNIX_X_SERVER_SOCKET_PATH, display.display
    ))?;

    Connection::setup(Box::new(connection))
}

pub trait StreamMarker: Read + Write {}
impl<T: Read + Write> StreamMarker for T {}

/// Xodium connection to X server.
/// Works over any type implementing [Read] + [Write].
pub struct Connection {
    _socket: Box<dyn StreamMarker>,
}

impl Connection {
    /// Setup connection over any type implementing [Read] + [Write]
    pub fn setup(_socket: Box<dyn StreamMarker>) -> Result<Self, ConnectionError> {
        Ok(Connection { _socket })
    }
}
