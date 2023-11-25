#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

use crate::FormatError;

pub(crate) const PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];

pub(crate) const DECIMAL_CHAR: char = '.';
pub(crate) const GROUP_DELIMITER_CHAR: char = ',';

// default number format settings
pub(crate) const DEFAULT_ZERO_PADDING: bool = true;
pub(crate) const DEFAULT_FILL: char = ' ';
pub(crate) const DEFAULT_ALIGN: NumberAlign = NumberAlign::Right;
pub(crate) const DEFAULT_SIGN: Sign = Sign::OnlyNegative;
pub(crate) const DEFAULT_TYPE_PREFIX: bool = false;
pub(crate) const DEFAULT_MIN_WIDTH: usize = 0;
pub(crate) const DEFAULT_MAX_WIDTH: usize = usize::MAX;
pub(crate) const DEFAULT_COMMAS: bool = false;
pub(crate) const DEFAULT_PRECISION: usize = 6;
pub(crate) const DEFAULT_FORMAT_TYPE: FormatType = FormatType::None;

/// Represents a destructured specification of a provided format pattern string.
#[derive(Debug, Clone)]
pub struct NumberFormat {
    /// zero padding
    pub zero_padding: bool,
    /// fill character
    pub fill: char,
    /// alignment
    pub align: NumberAlign,
    /// sign
    pub sign: Sign,
    /// type prefix
    pub type_prefix: bool,
    /// min_width
    pub min_width: usize,
    /// max_width
    pub max_width: usize,
    /// commas
    pub commas: bool,
    /// decimals
    pub precision: usize,
    /// format type
    pub format_type: FormatType,
}

impl NumberFormat {
    /// format number value
    pub fn format<T: Into<f64>>(&self, input: T) -> Result<String, FormatError> {
        super::interface::number_format(self, input)
    }
}

impl Default for NumberFormat {
    fn default() -> NumberFormat {
        NumberFormat {
            zero_padding: DEFAULT_ZERO_PADDING,
            fill: DEFAULT_FILL,
            align: NumberAlign::default(),
            sign: Sign::default(),
            type_prefix: DEFAULT_TYPE_PREFIX,
            min_width: DEFAULT_MIN_WIDTH,
            max_width: DEFAULT_MAX_WIDTH,
            commas: DEFAULT_COMMAS,
            precision: DEFAULT_PRECISION,
            format_type: FormatType::default(),
        }
    }
}

/// NumberAlignment
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NumberAlign {
    /// left align
    Left,
    /// right align
    Right,
    /// center align
    Center,
    /// center align with sign on left side
    SignedRight,
}

impl Default for NumberAlign {
    fn default() -> NumberAlign {
        DEFAULT_ALIGN
    }
}

/// Whether to include sign
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    /// only show sign when negative
    OnlyNegative,
    /// always show sign
    Always,
    /// space or dash
    SpaceOrDash,
}

impl Default for Sign {
    fn default() -> Sign {
        DEFAULT_SIGN
    }
}

/// format type
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FormatType {
    /// exponent format
    Exponent,
    /// exponent upper case format
    ExponentUppercase,
    /// fixed point format
    FixedPoint,
    /// SI prefix format
    SI,
    /// percentage format
    Percentage,
    /// binary format
    Binary,
    /// octal format
    Octal,
    /// octal upper case format
    OctalUppercase,
    /// decimal format
    Decimal,
    /// hex format
    Hex,
    /// hex upper case format
    HexUppercase,
    /// no format
    None,
}

impl FormatType {
    /// list all FormatType variants
    pub fn all_variants() -> Vec<FormatType> {
        vec![
            FormatType::Exponent,
            FormatType::ExponentUppercase,
            FormatType::FixedPoint,
            FormatType::SI,
            FormatType::Percentage,
            FormatType::Binary,
            FormatType::Octal,
            FormatType::OctalUppercase,
            FormatType::Decimal,
            FormatType::Hex,
            FormatType::HexUppercase,
            FormatType::None,
        ]
    }
}

impl Default for FormatType {
    fn default() -> FormatType {
        DEFAULT_FORMAT_TYPE
    }
}
