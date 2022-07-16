use std::{
    borrow::Borrow,
    io::{Read, Seek, SeekFrom},
};

use super::super::FormatType;
use crate::{audio_util, error::CksError, file_header::FileHeader, sample::info::SampleInfo};
pub struct DecoderCore<R>
where
    R: Seek + Read,
{
    reader: R,
    pub header: FileHeader,
    pub sample_info: SampleInfo,
    pub stream_size: u64,
    frame_starts: u64,
    reader_buf: Vec<u8>,
    audio_util_buf: audio_util::AudioUtil,
}

impl<R> DecoderCore<R>
where
    R: Seek + Read,
{
    pub fn new(mut reader: R) -> Result<Self, CksError> {
        let current_pos = reader.seek(SeekFrom::Current(0)).unwrap();
        let stream_size = reader.seek(SeekFrom::End(0)).unwrap();
        if current_pos != 0 {
            let _ = reader.seek(SeekFrom::Start(current_pos));
        } else {
            let _ = reader.rewind();
        }
        let header = FileHeader::new(reader.by_ref())?;
        let sample_info = SampleInfo::new(reader.by_ref());
        let frame_starts = reader.stream_position().unwrap();
        let reader_buf = Vec::with_capacity(sample_info.block_bytes as usize * 2);
        let audio_util_buf = audio_util::AudioUtil::new();
        Ok(Self {
            reader,
            header,
            sample_info,
            stream_size,
            frame_starts,
            reader_buf,
            audio_util_buf,
        })
    }

    //return amount of frames which read.
    pub fn decode(&mut self, buf: &mut FormatType, blocks: i32) -> Option<u64> {
        if self.is_done() {
            //no frames to read.
            None
        } else {
            //let channels = self.sample_info.channels;
            let frames_read = self.read(blocks);
            //println!("read: {:?}", self.reader_buf);
            match buf {
                FormatType::Int32(buf_i32_v) => {
                    match self.sample_info.format {
                        //crate::decoder::DecoderType::Adpcm => todo!(),
                        crate::decoder::DecoderType::Pcmi8 => {
                            self.audio_util_buf.convert_i8_to_i32(&self.reader_buf, buf_i32_v)
                        }
                        crate::decoder::DecoderType::Pcmi16 => {
                            self.audio_util_buf.convert_i16_to_i32(&self.reader_buf, buf_i32_v)
                        }
                        crate::decoder::DecoderType::Pcmf32 => {
                            self.audio_util_buf.convert_f_to_i32(&self.reader_buf, buf_i32_v)
                        }
                        //crate::decoder::DecoderType::Unknown => todo!(),
                        _ => return None,
                    }
                }
                FormatType::Float(buf_f32_v) => {
                    match self.sample_info.format {
                        //crate::decoder::DecoderType::Adpcm => todo!(),
                        crate::decoder::DecoderType::Pcmi8 => {
                            self.audio_util_buf.convert_i8_f(&self.reader_buf, buf_f32_v)
                        }
                        crate::decoder::DecoderType::Pcmi16 => {
                            self.audio_util_buf.convert_i16_to_f(&self.reader_buf, buf_f32_v)
                        }
                        crate::decoder::DecoderType::Pcmf32 => {
                            self.audio_util_buf.convert_f_to_f(&self.reader_buf, buf_f32_v)
                        }
                        //crate::decoder::DecoderType::Unknown => todo!(),
                        _ => return None,
                    }
                }
            }
            Some(frames_read)
        }
    }
    //fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32;
    //fn decode_float(&self, buf: &mut f32, frames: i32) -> i32;
    fn is_done(&mut self) -> bool {
        let current_pos = self.reader.stream_position().unwrap();
        current_pos >= self.stream_size
    }

    /*
    fn read_i32(&mut self, buf: &mut Vec<i32>, blocks: i32) -> u64 {
        let bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end =
            std::cmp::max(self.stream_size - self.reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end);
        if bytes_to_read > 0 {
            let mut buf_f = [0_u8; 4];
            let size_needed = (bytes_to_read / 4) as usize;
            if buf.len() <= size_needed {
                buf.resize(size_needed, 0);
            }
            for i in 0..size_needed {
                self.reader.read_exact(&mut buf_f);
                buf[i] = i32::from_be_bytes(buf_f);
            }
        }
        bytes_to_read
    }
    */

    fn read(&mut self, blocks: i32) -> u64 {
        let buf = &mut self.reader_buf;
        let bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end =
            std::cmp::max(self.stream_size - self.reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end) as usize;
        //println!("{}", bytes_to_read);
        if bytes_to_read > 0 {
            //let mut buf_f = [0_u8; 4];
            if buf.len() <= bytes_to_read {
                buf.resize(bytes_to_read, 0);
            }
            self.reader.read(&mut buf[0..bytes_to_read]).unwrap();

            /*
            for i in 0..size_needed {
                self.reader.read_exact(&mut buf_f).unwrap();
                buf[i] = f32::from_be_bytes(buf_f);
            }
            */
        }
        bytes_to_read as u64
    }

    //frame starts with 0.
    pub fn set_frame_pos(&mut self, frame: i32) {
        let _ = self.reader.seek(SeekFrom::Start(
            self.frame_starts + (frame as u64 * self.sample_info.block_bytes as u64),
        ));
    }

    pub fn get_frame_pos(&mut self) -> u64 {
        self.reader.stream_position().unwrap() / (self.sample_info.block_bytes as u64)
    }

    pub fn get_num_frames(&mut self) -> u64 {
        let current_pos = self.reader.stream_position().unwrap();
        ((current_pos as i64 - self.frame_starts as i64) / (self.sample_info.block_bytes as i64))
            as u64
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}
