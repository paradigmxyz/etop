/// format error
#[derive(Debug)]
pub enum FormatError {
    /// could not parse format type
    CouldNotParseFormatType,
    /// could not decompose coefficient exponent
    CouldNotDecomposeCoefficientExponent,
    /// could not create regex
    CouldNotCreateRegex,
    /// regex could not match
    CouldNotMatchRegex,
    /// invalid format
    InvalidFormat(String),
    /// empty data
    EmptyData(String),
    /// polars erro
    PolarsError(polars::prelude::PolarsError),
    /// column missing
    ColumnMissing(String),
    /// unsupported datatype
    UnsupportedDatatype(String),
    /// mismatched format type
    MismatchedFormatType(String),
}

impl From<polars::prelude::PolarsError> for FormatError {
    fn from(err: polars::prelude::PolarsError) -> FormatError {
        FormatError::PolarsError(err)
    }
}
