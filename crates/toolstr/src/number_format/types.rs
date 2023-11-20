#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

pub(crate) const PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

pub(crate) const DECIMAL_CHAR: char = '.';
pub(crate) const GROUP_DELIMITER_CHAR: char = ',';

// default number format settings
pub(crate) const DEFAULT_ZERO_PADDING: bool = true;
pub(crate) const DEFAULT_FILL: char = ' ';
pub(crate) const DEFAULT_ALIGN: Align = Align::Right;
pub(crate) const DEFAULT_SIGN: Sign = Sign::OnlyNegative;
pub(crate) const DEFAULT_TYPE_PREFIX: bool = false;
pub(crate) const DEFAULT_WIDTH: usize = 0;
pub(crate) const DEFAULT_COMMAS: bool = false;
pub(crate) const DEFAULT_PRECISION: usize = 6;
pub(crate) const DEFAULT_FORMAT_TYPE: FormatType = FormatType::None;

/// Represents a destructured specification of a provided format pattern string.
#[derive(Debug)]
pub struct NumberFormat {
    /// zero padding
    pub zero_padding: bool,
    /// fill character
    pub fill: char,
    /// alignment
    pub align: Align,
    /// sign
    pub sign: Sign,
    /// type prefix
    pub type_prefix: bool,
    /// width
    pub width: usize,
    /// commas
    pub commas: bool,
    /// decimals
    pub precision: usize,
    /// format type
    pub format_type: FormatType,
}

// impl NumberFormat {
//     /// format number value
//     pub fn format<T: Into<f64>>(&self, value: T) -> Result<String, FormatError> {
//         super::interface::format(self, value)
//     }
// }

impl Default for NumberFormat {
    fn default() -> NumberFormat {
        NumberFormat {
            zero_padding: DEFAULT_ZERO_PADDING,
            fill: DEFAULT_FILL,
            align: Align::default(),
            sign: Sign::default(),
            type_prefix: DEFAULT_TYPE_PREFIX,
            width: DEFAULT_WIDTH,
            commas: DEFAULT_COMMAS,
            precision: DEFAULT_PRECISION,
            format_type: FormatType::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Default for Align {
    fn default() -> Align {
        DEFAULT_ALIGN
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    OnlyNegative,
    Always,
    SpaceOrDash,
}

impl Default for Sign {
    fn default() -> Sign {
        DEFAULT_SIGN
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Default for FormatType {
    fn default() -> FormatType {
        DEFAULT_FORMAT_TYPE
    }
}

#[derive(Debug)]
pub enum FormatError {
    CouldNotParseFormatType,
    CouldNotDecomposeCoefficientExponent,
}

