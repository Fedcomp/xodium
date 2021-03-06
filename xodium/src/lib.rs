#![forbid(unsafe_code)]

//! Xodium is a pure rust X11 client.
//!
//! Opening connection is as easy as:
//! ```no_run
//! // Connect to X server using DISPLAY environment variable
//! let connection = xodium::connect_default();
//! ```

mod connection;
mod display;
mod framed;
mod protocol;
mod utils;
mod xauthority;

pub use connection::{connect_default, connect_to_display, Connection, ConnectionError};
pub use display::Display;
pub use utils::StreamMarker;
