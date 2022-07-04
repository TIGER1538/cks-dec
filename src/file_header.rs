use crate::error as cks_err;
use std::io::{Read, Seek};

pub struct FileHeader<R> {
    reader: R,
    marker: String,
    targets: u32,
    file_type: u32,
    file_version: u32,
}

impl<R> FileHeader<R>
where
    R: Seek + Read,
{
    pub fn new(mut reader: R) -> Result<Self, cks_err::Error> {
        let mut buffer_unit = [0u8; 4];
        let mut marker = String::with_capacity(4);
        let mut targets = 0;
        let mut file_type = 0;
        let mut file_version = 0;

        reader.read_exact(&mut buffer_unit).or(Err(cks_err::Error::FileRead))?;
        for i in 0..4 {
            marker.push(buffer_unit[i] as char);
        }

        reader.read_exact(&mut buffer_unit).or(Err(cks_err::Error::FileRead))?;
        write_header_info(&buffer_unit, &mut targets);
        reader.read_exact(&mut buffer_unit).or(Err(cks_err::Error::FileRead))?;
        write_header_info(&buffer_unit, &mut file_type);
        reader.read_exact(&mut buffer_unit).or(Err(cks_err::Error::FileRead))?;
        write_header_info(&buffer_unit, &mut file_version);

        Ok(Self {
            reader,
            marker,
            targets,
            file_type,
            file_version,
        })
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}

fn write_header_info(buf_read: &[u8; 4], target: &mut u32) {
    *target = u32::from_le_bytes(*buf_read);
}
