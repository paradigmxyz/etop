#[derive(Debug)]
pub enum EtopError {
    ArgumentError(String),
    ParseError(String),
    CouldNotOpenFile(String),
    CouldNotReadFile(String),
    InvalidFormat(String),
    PolarsError(polars::prelude::PolarsError),
    ColumnMissing(String),
    MissingData(String),
    UnknownData(String),
    UnsupportedDatatype(String),
    MismatchedFormatType(String),
    FormatError(toolstr::FormatError),
    GlobError(glob::PatternError),
    IOError(std::io::Error),
}

impl From<polars::prelude::PolarsError> for EtopError {
    fn from(err: polars::prelude::PolarsError) -> EtopError {
        EtopError::PolarsError(err)
    }
}

impl From<toolstr::FormatError> for EtopError {
    fn from(err: toolstr::FormatError) -> EtopError {
        EtopError::FormatError(err)
    }
}

impl From<glob::PatternError> for EtopError {
    fn from(err: glob::PatternError) -> EtopError {
        EtopError::GlobError(err)
    }
}

impl From<glob::GlobError> for EtopError {
    fn from(err: glob::GlobError) -> EtopError {
        EtopError::IOError(err.into_error())
    }
}
