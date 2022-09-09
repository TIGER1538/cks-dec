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
        let mut buf = ReaderBuf {
            buf_4bytes: [0, 0, 0, 0],
            buf_2bytes: [0, 0],
            buf_1byte: [0],
        };
        let format = read_to_u8(&mut reader, &mut buf);
        let channels = read_to_u8(&mut reader, &mut buf);
        let sample_rate = read_to_u16(&mut reader, &mut buf);
        let blocks = read_to_i32(&mut reader, &mut buf);
        let block_bytes = read_to_u16(&mut reader, &mut buf);
        let block_frames = read_to_u16(&mut reader, &mut buf);
        let volume = read_to_u16(&mut reader, &mut buf);
        let pan = read_to_i16(&mut reader, &mut buf);
        let loop_start = read_to_u32(&mut reader, &mut buf);
        let loop_end = read_to_u32(&mut reader, &mut buf);
        let loop_count = read_to_i16(&mut reader, &mut buf);
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

struct ReaderBuf {
    buf_4bytes: [u8; 4],
    buf_2bytes: [u8; 2],
    buf_1byte: [u8; 1],
}

#[inline]
fn read_to_u8<R>(reader: &mut R, buf: &mut ReaderBuf) -> u8
where
    R: Read + Seek,
{
    let mut buf = buf.buf_1byte;
    let _ = reader.read_exact(&mut buf);
    buf[0]
}

#[inline]
fn read_to_u16<R>(reader: &mut R, buf: &mut ReaderBuf) -> u16
where
    R: Read + Seek,
{
    let mut buf = buf.buf_2bytes;
    let _ = reader.read_exact(&mut buf);
    u16::from_le_bytes(buf)
}

#[inline]
fn read_to_u32<R>(reader: &mut R, buf: &mut ReaderBuf) -> u32
where
    R: Read + Seek,
{
    let mut buf = buf.buf_4bytes;
    let _ = reader.read_exact(&mut buf);
    u32::from_le_bytes(buf)
}

#[inline]
fn read_to_i16<R>(reader: &mut R, buf: &ReaderBuf) -> i16
where
    R: Read + Seek,
{
    let mut buf = buf.buf_2bytes;
    let _ = reader.read_exact(&mut buf);
    i16::from_le_bytes(buf)
}

#[inline]
fn read_to_i32<R>(reader: &mut R, buf: &mut ReaderBuf) -> i32
where
    R: Read + Seek,
{
    let mut buf = buf.buf_4bytes;
    let _ = reader.read_exact(&mut buf);
    i32::from_le_bytes(buf)
}
