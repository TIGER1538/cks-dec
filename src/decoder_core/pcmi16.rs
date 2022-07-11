use std::{
    cell::RefCell,
    io::{Read, Seek},
    rc::Rc,
};

use super::CksError;
use super::DecoderTrait;
use crate::{file_header::FileHeader, sample::info::SampleInfo};

pub struct Pcmi16<R> {
    reader: Rc<RefCell<R>>,
    header: FileHeader<R>,
    sample_info: SampleInfo<R>,
}

impl<R> DecoderTrait<R> for Pcmi16<R>
where
    R: Read + Seek,
{
    fn new(reader: R) -> Result<Pcmi16<R>, CksError> {
        let reader_rc_cell = Rc::new(RefCell::new(reader));
        let header = FileHeader::new(Rc::clone(&reader_rc_cell))?;
        let sample_info = SampleInfo::new(Rc::clone(&reader_rc_cell));
        Ok(Self {
            reader: reader_rc_cell,
            header,
            sample_info,
        })
    }

    fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32 {
        if self.is_done() {
            return 0;
        } else {
            let _reader = self.reader.borrow_mut();
            let frames_read = self.sample_info.
            todo!()
        }
    }

    fn decode_float(&self, buf: &mut f32, frames: i32) -> i32 {
        todo!()
    }

    fn is_done(&self) -> bool {
        todo!()
    }

    fn set_frame_pos(&self, frame: i32) {
        todo!()
    }

    fn get_frame_pos(&self) -> i32 {
        todo!()
    }

    fn get_num_frames(&self) -> i32 {
        todo!()
    }
}

impl Pcmi16<R> {
    fn decode() -> Result<i32, CksError> {

    }
}
