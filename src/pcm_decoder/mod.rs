pub mod pcmi8;
pub mod pcmi16;
pub mod pcmf32;
pub mod adpcm;

trait DecoderTrait<R> {
    fn new(reader: R);
    fn decode_i32(&self, buf: &i32, frames: i32) -> i32;
    fn decode_float(&self, buf: &f32, frames: i32) -> i32;
    fn is_done(&self) -> bool;
    fn set_frame_pos(&self, frame: i32);
    fn get_frame_pos(&self) -> i32;
    fn get_num_frames(&self) -> i32;
}