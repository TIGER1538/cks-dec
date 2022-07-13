pub mod decoder;
pub mod sample;
mod error;
mod decoder_core;
mod file_header;
mod audio_util;

pub enum FormatType {
    Int32(Vec<i32>),
    Float(Vec<f32>),
}