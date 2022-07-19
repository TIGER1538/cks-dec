pub mod decoder;
pub mod sample;
mod error;
mod decoder_core;
mod file_header;
mod audio_util;

pub enum FormatType {
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Float(Vec<f32>),
}