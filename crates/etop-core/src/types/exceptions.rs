/// etop error
#[derive(Debug)]
pub enum EtopError {
    /// argument error
    ArgumentError(String),
    /// parse error
    ParseError(String),
    /// could not open file
    CouldNotOpenFile(String),
    /// could not read file
    CouldNotReadFile(String),
    /// invalid format
    InvalidFormat(String),
    /// polars error
    PolarsError(polars::prelude::PolarsError),
    /// columns missing
    ColumnMissing(String),
    /// missing data
    MissingData(String),
    /// unknown data
    UnknownData(String),
    /// empty data
    EmptyData(String),
    /// unsupported datatype
    UnsupportedDatatype(String),
    /// mismatched format type
    MismatchedFormatType(String),
    /// format error
    FormatError(etop_format::FormatError),
    /// glob error
    GlobError(glob::PatternError),
    /// io error
    IOError(std::io::Error),
    /// tui error
    TuiError(String),
}

impl From<polars::prelude::PolarsError> for EtopError {
    fn from(err: polars::prelude::PolarsError) -> EtopError {
        EtopError::PolarsError(err)
    }
}

impl From<etop_format::FormatError> for EtopError {
    fn from(err: etop_format::FormatError) -> EtopError {
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

impl From<std::io::Error> for EtopError {
    fn from(err: std::io::Error) -> EtopError {
        EtopError::IOError(err)
    }
}
