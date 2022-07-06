pub mod adpcm;
pub mod pcmf32;
pub mod pcmi16;
pub mod pcmi8;

use super::error::Error as cks_error;

trait DecoderTrait<R> {
    fn new(reader: R) -> Result<Self, cks_error>
    where
        Self: Sized;
    fn decode_i32(&self, buf: &mut i32, frames: i32) -> i32;
    fn decode_float(&self, buf: &mut f32, frames: i32) -> i32;
    fn is_done(&self) -> bool;
    fn set_frame_pos(&self, frame: i32);
    fn get_frame_pos(&self) -> i32;
    fn get_num_frames(&self) -> i32;
}
