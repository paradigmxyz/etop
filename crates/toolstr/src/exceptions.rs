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
}
