use std::fmt;
#[derive(Debug)]
pub enum ConvertError {
    Io(std::io::Error), Zip(String), Xml(String), Unsupported(String), Encrypted(String),
    Corrupt(String), Config(String), Output(String), Ocr(String), Other(String),
}
impl ConvertError { pub fn is_fatal(&self) -> bool { true } }
impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (kind, detail) = match self {
            Self::Io(e) => return write!(f, "I/O error: {}", e.kind()),
            Self::Zip(s) => ("ZIP error",s), Self::Xml(s) => ("XML error",s),
            Self::Unsupported(s) => ("unsupported format",s), Self::Encrypted(s) => ("encrypted document",s),
            Self::Corrupt(s) => ("corrupt document",s), Self::Config(s) => ("configuration error",s),
            Self::Output(s) => ("output error",s), Self::Ocr(s) => ("OCR error",s), Self::Other(s) => ("conversion error",s),
        };
        // Callers must pass controlled diagnostics, not library errors containing paths.
        write!(f, "{kind}: {detail}")
    }
}
impl std::error::Error for ConvertError {}
impl From<std::io::Error> for ConvertError { fn from(e: std::io::Error) -> Self { Self::Io(e) } }
pub type Result<T> = std::result::Result<T, ConvertError>;
