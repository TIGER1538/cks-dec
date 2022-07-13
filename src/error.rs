pub enum CksError {
    FileRead,
    NotCksFile,
    UnknownFormat,
    Io,
    InsufficientData,
    SkippedData,
    EoF,
}