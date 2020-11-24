use std::io::{self, Read, Write};

pub trait StreamMarker: Read + Write {}
impl<T: Read + Write> StreamMarker for T {}

pub(crate) trait Encoder {
    type Item;
    type Error: From<io::Error>;

    fn encode(&mut self, item: Self::Item, dst: &mut Vec<u8>) -> Result<(), Self::Error>;
}

pub(crate) trait Decoder {
    type Item;
    type Error: From<io::Error>;

    fn decode(&mut self, src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error>;
}

pub(crate) struct Framed<C: Encoder + Decoder> {
    stream: Box<dyn StreamMarker>,
    codec: C,
    read_buffer: Vec<u8>,
}

impl<C: Encoder + Decoder> Framed<C> {
    pub fn new(stream: Box<dyn StreamMarker>, codec: C) -> Framed<C> {
        Framed {
            stream,
            codec,
            read_buffer: Default::default(),
        }
    }

    pub fn next(&mut self) -> Result<<C as Decoder>::Item, <C as Decoder>::Error> {
        loop {
            match self.codec.decode(&mut self.read_buffer) {
                Ok(Some(v)) => return Ok(v),
                Ok(None) => {
                    let mut buf = [0u8; 1024];
                    let size = self.stream.read(&mut buf)?;

                    if size == 0 {
                        Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "Unexpected end while reading framed stream",
                        ))?;
                    }

                    self.read_buffer.extend(&buf[0..size]);
                }
                Err(err) => return Err(err),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Decoder, Encoder, Framed};
    use std::io::{self, Cursor};

    struct LinesCodec;

    impl Encoder for LinesCodec {
        type Item = Vec<u8>;
        type Error = io::Error;

        fn encode(&mut self, item: Self::Item, dst: &mut Vec<u8>) -> Result<(), Self::Error> {
            dst.extend(item);
            Ok(())
        }
    }

    impl Decoder for LinesCodec {
        type Item = Vec<u8>;
        type Error = io::Error;

        fn decode(&mut self, src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error> {
            let newline_position = match src.iter().position(|c| *c == b'\n') {
                Some(p) => p,
                None => return Ok(None),
            };

            let line: Vec<u8> = src
                .drain(0..=newline_position)
                .take_while(|c| *c != b'\n')
                .collect();

            Ok(Some(line))
        }
    }

    #[test]
    fn test_framed_next() {
        let stream = Cursor::new("line1\nline2\nline3".as_bytes().to_vec());
        let codec = LinesCodec;
        let mut framed = Framed::new(Box::new(stream), codec);
        assert_eq!(framed.next().unwrap(), b"line1");
        assert_eq!(framed.next().unwrap(), b"line2");
        assert!(framed.next().is_err());
    }
}
