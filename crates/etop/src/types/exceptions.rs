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
