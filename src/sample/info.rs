use std::io::{Read, Seek, SeekFrom};

use crate::decoder::DecoderType;

#[derive(Clone, Debug)]
pub struct SampleInfo {
    pub format: DecoderType,
    pub channels: u8,
    pub sample_rate: u16,
    pub blocks: i32, // -1 means unknown
    pub block_bytes: u16,
    pub block_frames: u16,

    pub volume: u16,
    pub pan: i16,

    pub loop_start: u32, // default 0
    pub loop_end: u32,   // default blocks * blockFrames
    pub loop_count: i16, // default 0
}

impl SampleInfo {
    pub fn new<R: Read + Seek>(mut reader: R) -> Self {
        let format = read_to_u8(&mut reader);
        let channels = read_to_u8(&mut reader);
        let sample_rate = read_to_u16(&mut reader);
        let blocks = read_to_i32(&mut reader);
        let block_bytes = read_to_u16(&mut reader);
        let block_frames = read_to_u16(&mut reader);
        let volume = read_to_u16(&mut reader);
        let pan = read_to_i16(&mut reader);
        let loop_start = read_to_u32(&mut reader);
        let loop_end = read_to_u32(&mut reader);
        let loop_count = read_to_i16(&mut reader);
        let _ = reader.seek(SeekFrom::Current(2));

        let format = match format {
            0 => DecoderType::Pcmi16,
            1 => DecoderType::Pcmi8,
            2 => DecoderType::Adpcm,
            3 => DecoderType::Pcmf32,
            _ => DecoderType::Unknown,
        };

        Self {
            format,
            channels,
            sample_rate,
            blocks,
            block_bytes,
            block_frames,
            volume,
            pan,
            loop_start,
            loop_end,
            loop_count,
        }
    }

    pub fn reset(self) -> Self {
        Self {
            format: DecoderType::Unknown,
            channels: 0,
            sample_rate: 0,
            blocks: 0,
            block_bytes: 0,
            block_frames: 0,
            volume: u16::max_value(),
            pan: 0,
            loop_start: 0,
            loop_end: u32::max_value(),
            loop_count: 0,
        }
    }
}

#[inline]
fn read_to_u8<R>(reader: &mut R) -> u8
where
    R: Read + Seek,
{
    let mut buf = [0u8; 1];
    let _ = (*reader).read_exact(&mut buf);
    buf[0]
}

#[inline]
fn read_to_u16<R>(reader: &mut R) -> u16
where
    R: Read + Seek,
{
    let mut buf = [0u8; 2];
    let _ = reader.read_exact(&mut buf);
    u16::from_le_bytes(buf)
}

#[inline]
fn read_to_u32<R>(reader: &mut R) -> u32
where
    R: Read + Seek,
{
    let mut buf = [0u8; 4];
    let _ = reader.read_exact(&mut buf);
    u32::from_le_bytes(buf)
}

#[inline]
fn read_to_i16<R>(reader: &mut R) -> i16
where
    R: Read + Seek,
{
    let mut buf = [0u8; 2];
    let _ = reader.read_exact(&mut buf);
    i16::from_le_bytes(buf)
}

#[inline]
fn read_to_i32<R>(reader: &mut R) -> i32
where
    R: Read + Seek,
{
    let mut buf = [0u8; 4];
    let _ = reader.read_exact(&mut buf);
    i32::from_le_bytes(buf)
}
