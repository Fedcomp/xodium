use std::io::{Read, Write};

pub trait StreamMarker: Read + Write {}
impl<T: Read + Write> StreamMarker for T {}
