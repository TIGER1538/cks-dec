use std::io::{Read, Seek, SeekFrom};

use super::super::FormatType;
use crate::{
    audio_util, 
    decoder_core::adpcm::AdpcmCore, 
    error::CksError, 
    file_header::FileHeader,
    sample::info::SampleInfo,
};
pub struct DecoderCore<R>
where
    R: Seek + Read,
{
    pub(crate) reader: R,
    pub(crate) header: FileHeader,
    pub(crate) sample_info: SampleInfo,
    pub(crate) stream_size: u64,
    frame_starts: u64,
    reader_buf: Vec<u8>,
    audio_util_buf: audio_util::AudioUtil,
    pub(crate) adpcm_core: Option<AdpcmCore>,
}

impl<R> DecoderCore<R>
where
    R: Seek + Read,
{
    pub(crate) fn new(mut reader: R) -> Result<Self, CksError> {
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
            adpcm_core: None
        })
    }

    //return amount of frames which read.
    pub(crate) fn decode(&mut self, buf: &mut FormatType, blocks: i32) -> Option<u64> {
        if self.is_done() {
            //no frames to read.
            None
        } else {            
            if let FormatType::Int16(buf_i16_v) = buf {
                return Some(AdpcmCore::decode(self, buf_i16_v).unwrap() as _);
            }

            let frames_read = self.read(blocks);
            match buf {
                FormatType::Int32(buf_i32_v) => {
                    match self.sample_info.format {
                        crate::decoder::DecoderType::Pcmi8 => self
                            .audio_util_buf
                            .convert_i8_to_i32(&self.reader_buf, buf_i32_v),
                        crate::decoder::DecoderType::Pcmi16 => self
                            .audio_util_buf
                            .convert_i16_to_i32(&self.reader_buf, buf_i32_v),
                        crate::decoder::DecoderType::Pcmf32 => self
                            .audio_util_buf
                            .convert_f_to_i32(&self.reader_buf, buf_i32_v),
                        _ => return None,
                    }
                }
                FormatType::Float(buf_f32_v) => {
                    match self.sample_info.format {
                        crate::decoder::DecoderType::Pcmi8 => self
                            .audio_util_buf
                            .convert_i8_f(&self.reader_buf, buf_f32_v),
                        crate::decoder::DecoderType::Pcmi16 => self
                            .audio_util_buf
                            .convert_i16_to_f(&self.reader_buf, buf_f32_v),
                        crate::decoder::DecoderType::Pcmf32 => self
                            .audio_util_buf
                            .convert_f_to_f(&self.reader_buf, buf_f32_v),
                        _ => return None,
                    }
                }
                _ => {return None;}
            }
            frames_read
        }
    }

    fn is_done(&mut self) -> bool {
        let current_pos = self.reader.stream_position().unwrap();
        current_pos >= self.stream_size
    }

    pub(crate) fn read(&mut self, blocks: i32) -> Option<u64> {
        let buf = &mut self.reader_buf;
        let bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end =
            std::cmp::max(self.stream_size - self.reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end) as usize;
        if bytes_to_read > 0 {
            if buf.len() < bytes_to_read {
                buf.resize(bytes_to_read, 0);
            }
            'check_bytes: while let Ok(bytes_read) = self.reader.read(&mut buf[0..bytes_to_read]) {
                if bytes_read != bytes_to_read {
                    self.reader.seek(SeekFrom::Current(-(bytes_read as i64))).unwrap();
                } else {
                    break 'check_bytes;
                }
            };
        } else {
            return None;
        }
        Some(bytes_to_read as u64)
    }

    //frame starts with 0.
    pub(crate) fn set_frame_pos(&mut self, frame: i32) {
        let _ = self.reader.seek(SeekFrom::Start(
            self.frame_starts + (frame as u64 * self.sample_info.block_bytes as u64),
        ));
    }

    pub(crate) fn get_frame_pos(&mut self) -> u64 {
        self.reader.stream_position().unwrap() / (self.sample_info.block_bytes as u64)
    }

    pub(crate) fn get_num_frames(&mut self) -> u64 {
        let current_pos = self.reader.stream_position().unwrap();
        ((current_pos as i64 - self.frame_starts as i64) / (self.sample_info.block_bytes as i64))
            as u64
    }

    pub(crate) fn into_inner(self) -> R {
        self.reader
    }
}
