use crate::error::CksError;
use std::{
    cell::RefCell,
    io::{Read, Seek},
    rc::Rc,
};

pub struct FileHeader<R> {
    reader: Rc<RefCell<R>>,
    marker: String,
    targets: u32,
    file_type: u32,
    file_version: u32,
}

impl<R> FileHeader<R>
where
    R: Seek + Read,
{
    pub fn new(mut reader_rc: Rc<RefCell<R>>) -> Result<Self, CksError> {
        let mut reader = reader_rc.borrow_mut();
        let mut buffer_unit = [0u8; 4];
        let mut marker = String::with_capacity(4);
        let mut targets = 0;
        let mut file_type = 0;
        let mut file_version = 0;

        reader
            .read_exact(&mut buffer_unit)
            .or(Err(CksError::FileRead))?;
        for i in 0..4 {
            marker.push(buffer_unit[i] as char);
        }

        reader
            .read_exact(&mut buffer_unit)
            .or(Err(CksError::FileRead))?;
        write_header_info(&buffer_unit, &mut targets);
        reader
            .read_exact(&mut buffer_unit)
            .or(Err(CksError::FileRead))?;
        write_header_info(&buffer_unit, &mut file_type);
        reader
            .read_exact(&mut buffer_unit)
            .or(Err(CksError::FileRead))?;
        write_header_info(&buffer_unit, &mut file_version);
        drop(reader);

        Ok(Self {
            reader: reader_rc,
            marker,
            targets,
            file_type,
            file_version,
        })
    }
}

fn write_header_info(buf_read: &[u8; 4], target: &mut u32) {
    *target = u32::from_le_bytes(*buf_read);
}
