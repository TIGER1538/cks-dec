use std::io::{BufReader, Seek, Read};

use crate::error::CksError;
use crate::decoder_core::core::DecoderCore;

pub enum DecoderType
{
    Adpcm,
    Pcmi8,
    Pcmi16,
    Pcmf32,
    Unknown
}

pub struct Decoder<R>
where R: Read + Seek
{
    //reader: R,
    decorder: DecoderCore<R>,
    decoder_type: DecoderType
}

 /*
 pub trait DecoderTrait<R> {
    fn new(reader: R) -> Self;

    fn into_inner(s: Self) -> R
    where Self: Sized;

    /*
    fn decode_frame(&mut self) -> Result<Frame, Error> {

    }
    */
}
*/

impl<R> Decoder<R> 
where R: Read + Seek
{
    pub fn new(mut reader: R) -> Result<Self, CksError> {
        todo!()
    }

    pub fn decode(&self) -> Result<i32, CksError>{
        todo!()
    }

    pub fn next(&self) {

    }

    fn is_cks(reader: R) -> bool {
        let cks_marker = ['c' as u8, 'k' as u8, 'm' as u8, 'k' as u8];
        let mut buf = [0u8; 4];
        let current_pos = reader.stream_position().unwrap();
        if current_pos != 0 {
            let _ = reader.seek(std::io::SeekFrom::Start(0));
        }
        let _ = reader.read_exact(&mut buf);
        let _ = reader.seek(std::io::SeekFrom::Start(current_pos));
        cks_marker == buf
    }
}