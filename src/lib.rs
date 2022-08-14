#![allow(dead_code, unused)]
mod audio_util;
pub mod decoder;
mod decoder_core;
mod error;
mod file_header;
pub mod sample;

#[cfg(feature="time-stretch")]
pub mod time_stretch;

#[derive(PartialEq, Debug)]
pub enum FormatType {
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Float(Vec<f32>),
}

impl FormatType {
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            FormatType::Int16(v) => v.len(),
            FormatType::Int32(v) => v.len(),
            FormatType::Float(v) => v.len(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn new_int16() -> Self {
        let v_size = crate::decoder_core::adpcm::BYTES_PER_BLOCK_DEFAULT * 4 - 24;
        let v = vec![0; v_size];
        FormatType::Int16(v)
    }

    #[inline]
    pub fn new_int32() -> Self {
        let v_size = crate::decoder_core::adpcm::BYTES_PER_BLOCK_DEFAULT * 2;
        let v = vec![0; v_size];
        FormatType::Int32(v)
    }

    #[inline]
    pub fn new_float32() -> Self {
        let v_size = crate::decoder_core::adpcm::BYTES_PER_BLOCK_DEFAULT * 2;
        let v = vec![0.; v_size];
        FormatType::Float(v)
    }
}
