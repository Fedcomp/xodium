use std::io::{Read, Write};

/// Auto trait for any type implementing
/// [Read](std::io::Read) + [Write](std::io::Write).
/// Sole purpose of this trait is to
/// make dyn [Read](std::io::Read) + [Write](std::io::Write) kind of possible.
pub trait StreamMarker: Read + Write {}
impl<T: Read + Write> StreamMarker for T {}
