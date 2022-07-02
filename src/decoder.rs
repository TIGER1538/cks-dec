use crate::{frame::Frame, error::Error};
use crate::pcm_decoder::*;

pub enum Decoder<R> {
    Pcmi8(pcmi8::Pcmi8<R>),
    Pcmi16(pcmi16::Pcmi16<R>),
    Pcmf32(pcmf32::Pcmf32<R>),
    Unknown
}

pub trait DecoderTrait<R> {
    fn new(reader: R) -> Self;

    fn into_inner(s: Self) -> R
    where Self: Sized;

    /*
    fn decode_frame(&mut self) -> Result<Frame, Error> {

    }
    */
}

impl<R> Decoder<R> {
    pub fn new(reader: R) -> Decoder<R> {
        todo!()
    }
}