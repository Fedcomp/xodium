use crate::framed::{Decoder, Encoder};
use std::io;

#[derive(Default)]
pub struct SetupCodec {}

impl Encoder for SetupCodec {
    type Item = ();
    type Error = io::Error;

    fn encode(&mut self, _item: Self::Item, _dst: &mut Vec<u8>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Decoder for SetupCodec {
    type Item = ();
    type Error = io::Error;

    fn decode(&mut self, _src: &mut Vec<u8>) -> Result<Option<Self::Item>, Self::Error> {
        Ok(None)
    }
}
