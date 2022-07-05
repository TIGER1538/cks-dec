use std::cell::{RefCell, RefMut};
use std::io::{Read, Seek};
use std::rc::Rc;

pub struct SampleInfo<R> {
    reader: Rc<RefCell<R>>,
    format: u8,
    channels: u8,
    sample_rate: u16,
    blocks: i32, // -1 means unknown
    block_bytes: u16,
    block_frames: u16,

    volume: u16,
    pan: i16,

    loop_start: u32, // default 0
    loop_end: u32,   // default blocks * blockFrames
    loop_count: i16, // default 0
}

impl<R> SampleInfo<R>
where
    R: Seek + Read,
{
    pub fn new(reader_rc: Rc<RefCell<R>>) -> Self {
        let mut reader = reader_rc.borrow_mut();
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
        drop(reader);

        Self {
            reader: reader_rc,
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
            reader: self.reader,
            format: u8::max_value(),
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

    pub fn read(self) -> Self {
        let mut reader = self.reader.borrow_mut();
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
        drop(reader);

        Self {
            reader: self.reader,
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
}

fn read_to_u8<R>(reader: &mut RefMut<R>) -> u8
where
    R: Read + Seek,
{
    let mut buf = [0u8; 1];
    let _ = (*reader).read_exact(&mut buf);
    buf[0]
}

fn read_to_u16<R>(reader: &mut RefMut<R>) -> u16
where
    R: Read + Seek,
{
    let mut buf = [0u8; 2];
    let _ = reader.read_exact(&mut buf);
    u16::from_ne_bytes(buf)
}

fn read_to_u32<R>(reader: &mut RefMut<R>) -> u32
where
    R: Read + Seek,
{
    let mut buf = [0u8; 4];
    let _ = reader.read_exact(&mut buf);
    u32::from_ne_bytes(buf)
}

fn read_to_i16<R>(reader: &mut RefMut<R>) -> i16
where
    R: Read + Seek,
{
    let mut buf = [0u8; 2];
    let _ = reader.read_exact(&mut buf);
    i16::from_ne_bytes(buf)
}

fn read_to_i32<R>(reader: &mut RefMut<R>) -> i32
where
    R: Read + Seek,
{
    let mut buf = [0u8; 4];
    let _ = reader.read_exact(&mut buf);
    i32::from_ne_bytes(buf)
}
