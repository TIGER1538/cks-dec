pub enum CksError {
    FileRead,
    UnknownFormat,
    Io,
    InsufficientData,
    SkippedData,
    EoF,
}