#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

pub(crate) const PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];
pub(crate) const DECIMAL_CHAR: char = '.';
pub(crate) const GROUP_DELIMITER_CHAR: char = ',';

/// Represents a destructured specification of a provided format pattern string.
#[derive(Debug)]
pub struct FormatSpec {
    pub zero_padding: bool,
    pub fill: char,
    pub align: Align,
    pub sign: Sign,
    pub type_prefix: bool,
    pub width: Option<usize>,
    pub commas: bool,
    pub precision: Option<i32>,
    pub format_type: FormatType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Align {
    /// left align
    Left,
    /// right align
    Right,
    /// center align
    Center,
    /// center align with sign on left side
    SignedRight,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Sign {
    OnlyNegative,
    Always,
    SpaceOrDash,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FormatType {
    Exponent,
    ExponentUppercase,
    FixedPoint,
    SI,
    Percentage,
    Binary,
    Octal,
    OctalUppercase,
    Decimal,
    Hex,
    HexUppercase,
    None,
}

pub enum FormatError {
    CouldNotParseFormatType,
}
