use std::io::{Read, Seek};

use super::FormatType;
use crate::decoder_core::core::DecoderCore;
use crate::error::CksError;

#[derive(Clone, Debug)]
pub enum DecoderType {
    Adpcm,
    Pcmi8,
    Pcmi16,
    Pcmf32,
    Unknown,
}

pub struct Decoder<R>
where
    R: Read + Seek,
{
    decorder_core: DecoderCore<R>,
    decoder_type: DecoderType,
}

impl<R> Decoder<R>
where
    R: Read + Seek,
{
    pub fn new(mut reader: R) -> Result<Self, CksError> {
        let _ = reader.rewind();
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

    pub fn decode(&mut self, buf: &mut FormatType, blocks: usize) -> Option<u64> {
        self.decorder_core.decode(buf, blocks as i32)
    }

    pub fn next(&mut self, buf: &mut FormatType) -> Option<u64> {
        self.decorder_core.decode(buf, 1)
    }

    pub fn into_inner(self) -> R {
        self.decorder_core.into_inner()
    }

    pub fn sample_info(&self) -> crate::sample::info::SampleInfo {
        self.decorder_core.sample_info.clone()
    }

    fn is_cks(mut reader: R) -> bool {
        let cks_marker = [b'c', b'k', b'm', b'k'];
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

#[test]
fn t() {
    use std::io::Write;
    //use crate::audio_util::AudioUtil;
    let file_buf = std::io::BufReader::new(std::fs::File::open("components/BB0000.cks").unwrap());
    let out = std::fs::File::create("out.raw").unwrap();
    let mut out_b = std::io::BufWriter::new(out);
    let mut dec = Decoder::new(file_buf).unwrap();
    //let mut audio_util = AudioUtil::new();
    //let buf = vec![0_i16; 72];
    //let mut buf = FormatType::Int16(buf);
    let mut buf = FormatType::new_int16();

    //let mut current_frame = 0;
    while let Some(_) = dec.decode(&mut buf, 1) {
        if let FormatType::Int16(v) = &buf {
            //println!("{:#04X?}", v);
            for b in v.iter() {
                let _buf = b as *const i16 as *const [u8; 2];
                unsafe {
                    let _ = out_b.write(&*_buf);
                }
            }

            //AudioUtil::convert_i16_to_f(&mut audio_util, in_buf, out_buf);
            //println!("{:?}", v);
        }
    }
}

#[test]
fn v() {
    let buf0 = vec![0_i16; 72];
    let buf0 = FormatType::Int16(buf0);
    let buf1 = FormatType::new_int16();
    assert_eq!(buf0, buf1);
}
