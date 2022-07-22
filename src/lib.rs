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

impl FormatType {
    pub fn len(&self) -> usize {
        match self {
            FormatType::Int16(v) => v.len(),
            FormatType::Int32(v) => v.len(),
            FormatType::Float(v) => v.len(),
        }
    }
}