use std::io::{BufReader, Seek, Read};

use crate::decoder_core;
use crate::error::CksError;
use crate::decoder_core::core::DecoderCore;
use super::FormatType;

#[derive(Clone)]
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
    decorder_core: DecoderCore<R>,
    decoder_type: DecoderType
}

impl<R> Decoder<R> 
where R: Read + Seek
{
    pub fn new(mut reader: R) -> Result<Self, CksError> {
        if !Decoder::is_cks(reader.by_ref()) {
            return Err(CksError::NotCksFile);
        }
        let decorder_core = DecoderCore::new(reader)?;
        let decoder_type = decorder_core.sample_info.format.clone();

        Ok(Self {
            decorder_core,
            decoder_type,
        })
    }

    pub fn decode(&mut self, buf: &mut FormatType, blocks: usize) -> i32 {
        self.decorder_core.decode(buf, blocks as i32) as i32
    }

    pub fn next(&self) {

    }

    fn is_cks(mut reader: R) -> bool {
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