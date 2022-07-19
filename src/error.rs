#[derive(Debug)]
pub enum CksError {
    FileRead,
    NotCksFile,
    UnknownFormat,
    UnsupportedDecType,
    Io,
    InsufficientData,
    SkippedData,
    EoF,
}

#[derive(Debug)]
pub enum AdpcmError {
    InvalidStride,
    NoEnoughInputBytes,
}