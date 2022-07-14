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