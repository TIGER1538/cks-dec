use std::{
    cell::RefCell,
    io::{Read, Seek, SeekFrom},
    rc::Rc, array,
};

use crate::{error::CksError, file_header::FileHeader, sample::info::SampleInfo, audio_util};
use super::super::FormatType;
pub struct DecoderCore<R>
where
    R: Seek + Read,
{
    reader_rc_cell: Rc<RefCell<R>>,
    pub header: FileHeader<R>,
    pub sample_info: SampleInfo<R>,
    pub stream_size: u64,
    frame_starts: u64
}

impl<R> DecoderCore<R> 
where R: Seek +  Read
{
    pub fn new(mut reader: R) -> Result<Self, CksError> {
        let current_pos = reader.seek(SeekFrom::Current(0)).unwrap();
        let stream_size = reader.seek(SeekFrom::End(0)).unwrap();
        if current_pos != 0 {
            let _ = reader.seek(SeekFrom::Start(current_pos));
        }
        let reader_rc_cell = Rc::new(RefCell::new(reader));
        let header = FileHeader::new(Rc::clone(&reader_rc_cell))?;
        let sample_info = SampleInfo::new(Rc::clone(&reader_rc_cell));
        let frame_starts = (*reader_rc_cell.borrow_mut()).stream_position().unwrap();
        Ok(Self {
            reader_rc_cell,
            header,
            sample_info,
            stream_size,
            frame_starts
        })
    }

    //return amount of frames which read. 
    pub fn decode(&mut self, buf: &mut FormatType, blocks: i32) -> u64 {
        if self.is_done() {
            //no frames to read.
            0
        } else {
            //let channels = self.sample_info.channels;
            let mut frames_read = 0;
            match buf {
                FormatType::Int32(buf_i32_v) => {
                    frames_read = self.read_i32(buf_i32_v, blocks);
                },
                FormatType::Float(buf_f32_v) => {
                    frames_read = self.read_f32(buf_f32_v, blocks);
                },
            }
            frames_read
        }
    }
    //fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32;
    //fn decode_float(&self, buf: &mut f32, frames: i32) -> i32;
    fn is_done(&mut self) -> bool {
        let current_pos = self.reader_rc_cell.borrow_mut().stream_position().unwrap();
        current_pos >= self.stream_size
    }

    fn read_i32(&mut self, buf: &mut Vec<i32>, blocks: i32) -> u64 {
        let mut reader = self.reader_rc_cell.borrow_mut();
        let mut bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end = std::cmp::max(self.stream_size - reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end);
        if bytes_to_read > 0 {
            let mut buf_f = [0_u8; 4];
            let size_needed = (bytes_to_read/4) as usize;
            if buf.len() <= size_needed {
                buf.resize(size_needed, 0);
            }
            for i in 0..size_needed {
                reader.read_exact(&mut buf_f);
                buf[i] = i32::from_be_bytes(buf_f);
            }
        }
        bytes_to_read
    }

    fn read_f32(&mut self, buf: &mut Vec<f32>, blocks: i32) -> u64 {
        let mut reader = self.reader_rc_cell.borrow_mut();
        let bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end = std::cmp::max(self.stream_size - reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes as u64, bytes_to_end);
        if bytes_to_read > 0 {
            let mut buf_f = [0_u8; 4];
            let size_needed = (bytes_to_read/4) as usize;
            if buf.len() <= size_needed {
                buf.resize(size_needed, 0.0);
            }
            for i in 0..size_needed {
                reader.read_exact(&mut buf_f);
                buf[i] = f32::from_be_bytes(buf_f);
            }
        }
        bytes_to_read
    }
    //fn read_i32(&mut buf: i32, blocks: i32) -> i32;
    //fn read_float(&mut buf: i32, blocks: i32) -> i32;
    //frame starts with 0.
    pub fn set_frame_pos(&mut self, frame: i32) {
        self.reader_rc_cell.borrow_mut().seek(SeekFrom::Start(self.frame_starts + (frame as u64 * self.sample_info.block_bytes as u64)));
    }

    pub fn get_frame_pos(&mut self) -> u64 {
        self.reader_rc_cell.borrow_mut().stream_position().unwrap() / (self.sample_info.block_bytes as u64)
    }
    
    pub fn get_num_frames(&mut self) -> u64 {
        let current_pos = self.reader_rc_cell.borrow_mut().stream_position().unwrap();
        ((current_pos as i64 - self.frame_starts as i64) / (self.sample_info.block_bytes as i64)) as u64
    }
}
