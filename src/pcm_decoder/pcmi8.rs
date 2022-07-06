use super::DecoderTrait;

pub struct Pcmi8<R> {
    reader: R

}

impl<R> DecoderTrait<R> for Pcmi8<R> {
    fn new(reader: R) -> Result<Pcmi8<R>, crate::error::Error> {
        todo!()
    }

    fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32 {
        todo!()
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