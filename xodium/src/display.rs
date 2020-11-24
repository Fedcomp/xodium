use std::env::{self, VarError};
use std::fmt;

/// A `DISPLAY` environment variable type.
///
///
/// From man X(7):
///
/// From the user's perspective, every X server has a display name of the form:
/// ```quote
/// hostname:displaynumber.screennumber
/// ```
/// This information is used by the application to determine how it should
/// connect to the server and which screen it should use by default
/// (on displays with multiple monitors):
///
/// * **hostname** - The hostname specifies the name of the machine to which the
///   display is physically connected. If the hostname is not given, the most
///   efficient way of communicating to a server on the same machine will be
///   used.
///
/// * **displaynumber** - The phrase "display" is usually used to refer to
///   collection of monitors that share a common keyboard and pointer (mouse,
///   tablet, etc.). Most workstations tend to only have one keyboard, and
///   therefore, only one display. Larger, multi-user systems, however,
///   frequently have several displays so that more than one person can be doing
///   graphics work at once. To avoid confusion, each display on a machine is
///   assigned a display number (beginning at 0) when the X server for that
///   display is started. The display number must always be given in a display
///   name.
///
/// * **screennumber** - Some displays share a single keyboard and pointer
///   among two or more monitors. Since each monitor has its own set of windows,
///   each screen is assigned a screen number (beginning at 0) when the X server
///   for that display is started. If the screen number is not given, screen 0
///   will be used.
///
/// Examples of `DISPLAY` env:
/// ```quote
/// DISPLAY=:0
/// DISPLAY=:0.0
/// DISPLAY=host:0
/// DISPLAY=host:0.1
/// ```
///
/// *TODO:* Check validity of DISPLAY=localhost/unix:0
pub struct Display {
    pub hostname: Option<String>,
    pub display: u16,
    pub screen: Option<u16>,
}

impl Default for Display {
    fn default() -> Self {
        Display {
            hostname: None,
            display: 0,
            screen: None,
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            self.hostname.as_ref().map(String::as_str).unwrap_or(""),
            self.display
        )?;

        if let Some(screen) = self.screen {
            write!(f, ".{}", screen)?;
        }

        Ok(())
    }
}

impl Display {
    pub fn new(hostname: Option<String>, display: u16, screen: Option<u16>) -> Self {
        Self {
            hostname,
            display,
            screen,
        }
    }

    /// Try to parse DISPLAY string
    pub fn from_str(s: &str) -> Result<Self, DisplayError> {
        let hostname_end = s
            .find(|c| c == ':')
            .ok_or(DisplayError::InvalidDisplayFormat)?;
        let hostname = match &s[0..hostname_end] {
            "" => None,
            other => Some(other.into()),
        };
        let s = s
            .get(hostname_end + 1..)
            .ok_or(DisplayError::InvalidDisplayFormat)?;

        let screen_start = s.find(|c| c == '.');
        let display_end = screen_start.unwrap_or_else(|| s.len());
        let display = match &s[0..display_end] {
            "" => return Err(DisplayError::InvalidDisplayFormat),
            other => other
                .parse()
                .map_err(|_| DisplayError::InvalidDisplayFormat)?,
        };

        let screen = match screen_start {
            Some(p) => Some(
                s.get(p + 1..)
                    .ok_or(DisplayError::InvalidDisplayFormat)?
                    .parse()
                    .map_err(|_| DisplayError::InvalidDisplayFormat)?,
            ),
            None => None,
        };

        Ok(Display {
            hostname,
            display,
            screen,
        })
    }

    pub fn from_env() -> Result<Self, DisplayError> {
        let raw_display_value = match env::var("DISPLAY") {
            Ok(v) => v,
            Err(VarError::NotUnicode(_)) => return Err(DisplayError::InvalidDisplayFormat),
            Err(VarError::NotPresent) => return Err(DisplayError::DisplayNotSet),
        };

        Display::from_str(&raw_display_value)
    }
}

// TODO: Fmt
#[derive(Debug)]
pub enum DisplayError {
    InvalidDisplayFormat,
    DisplayNotSet,
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayError::InvalidDisplayFormat => write!(f, "Invalid DISPLAY format"),
            DisplayError::DisplayNotSet => write!(f, "DISPLAY variable not set"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Display, DisplayError};
    use std::env;

    #[test]
    fn test_display_from_str() {
        const EXAMPLES: &[&str] = &[":10", ":10.20", "hostname:10", "hostname:10.20"];

        for example in EXAMPLES {
            assert_eq!(Display::from_str(*example).unwrap().to_string(), *example);
        }

        const BAD_EXAMPLES: &[&str] = &[
            "",
            " ",
            ":",
            ".",
            ":.",
            "hostname:",
            "hostname:10.",
            ":10.",
            "hostname::10.20",
            "hostname:10.20.30",
            "hostname:.10",
        ];

        for bad_example in BAD_EXAMPLES {
            match Display::from_str(*bad_example) {
                Ok(_) => panic!(
                    "Display parsed invalid display string successfully!: {}",
                    bad_example
                ),
                Err(DisplayError::InvalidDisplayFormat) => {}
                Err(other) => panic!(
                    "Display used invalid error \"{}\" for {}",
                    other, bad_example
                ),
            };
        }
    }

    #[test]
    fn test_from_env() {
        const DISPLAY: &str = "hostname:10.20";
        let old_display_var = env::var_os("DISPLAY");

        env::set_var("DISPLAY", "hostname:10.20");
        let display = Display::from_env();
        match old_display_var {
            Some(d) => env::set_var("DISPLAY", d),
            None => env::remove_var("DISPLAY"),
        };

        assert_eq!(
            display.expect("DISPLAY should be parsed well").to_string(),
            DISPLAY
        );
    }

    #[test]
    fn test_default_display() {
        assert_eq!(Display::default().to_string(), ":0");
    }
}
