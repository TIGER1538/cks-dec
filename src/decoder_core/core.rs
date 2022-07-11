use std::{
    cell::RefCell,
    io::{Read, Seek, SeekFrom},
    rc::Rc,
};

use crate::{error::CksError, file_header::FileHeader, sample::info::SampleInfo};

pub struct DecoderCore<R>
where
    R: Seek + Read,
{
    reader: R,
    header: FileHeader<R>,
    sample_info: SampleInfo<R>,
    stream_size: u64,
}

impl<R> DecoderCore<R> 
where R: Seek +  Read
{
    fn new(reader: R) -> Result<Self, CksError> {
        let current_pos = reader.seek(SeekFrom::Current(0)).unwrap();
        let stream_size = reader.seek(SeekFrom::End(0)).unwrap();
        if current_pos != 0 {
            let _ = reader.seek(SeekFrom::Start(current_pos));
        }
        let reader_rc_cell = Rc::new(RefCell::new(reader));
        let header = FileHeader::new(Rc::clone(&reader_rc_cell))?;
        let sample_info = SampleInfo::new(Rc::clone(&reader_rc_cell));
        Ok(Self {
            reader,
            header,
            sample_info,
            stream_size
        })
    }

    //return amount of frames which read. 
    fn decode(&self, buf: &mut Vec<u8>) -> i32 {
        if self.is_done() {
            //no frames to read.
            return 0;
        } else {
            if 
            
        }
    }
    //fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32;
    //fn decode_float(&self, buf: &mut f32, frames: i32) -> i32;
    fn is_done(&self) -> bool {
        let current_pos = self.reader.stream_position().unwrap();
        current_pos >= self.stream_size
    }

    fn read(&self, buf: &mut Vec<u8>, blocks: i32) -> i32 {
        let mut bytes = blocks * self.sample_info.block_bytes as i32;
        let bytes_to_end = std::cmp::max(self.stream_size - self.reader.stream_position().unwrap(), 0);
        let bytes_to_read = std::cmp::min(bytes, bytes_to_end);
        if bytes_to_read > 0 {
            
        }
    }
    //fn read_i32(&mut buf: i32, blocks: i32) -> i32;
    //fn read_float(&mut buf: i32, blocks: i32) -> i32;
    fn set_frame_pos(&self, frame: i32);
    fn get_frame_pos(&self) -> i32;
    fn get_num_frames(&self) -> i32;
}
