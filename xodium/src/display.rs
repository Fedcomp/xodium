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
}

#[cfg(test)]
mod tests {
    use super::Display;

    #[test]
    fn test_display_fmt_hostname_and_display_and_screen() {
        let display = Display::new(Some("hostname".into()), 10, Some(20));
        assert_eq!(display.to_string(), "hostname:10.20");
    }

    #[test]
    fn test_display_fmt_display_and_screen() {
        let display = Display::new(None, 10, Some(20));
        assert_eq!(display.to_string(), ":10.20");
    }

    #[test]
    fn test_display_fmt_display() {
        let display = Display::new(None, 10, None);
        assert_eq!(display.to_string(), ":10");
    }
}
