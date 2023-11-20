use regex::Captures;
use regex::Regex;
use std::str::FromStr;

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
pub(crate) struct FormatSpec<'a> {
    pub zero_padding: bool,
    pub fill: Option<&'a str>,
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

impl FromStr for Align {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Align::Right),
            "<" => Ok(Align::Left),
            "^" => Ok(Align::Center),
            "=" => Ok(Align::SignedRight),
            _ => Err(FormatError::CouldNotParseFormatType),
        }
    }
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

impl FromStr for FormatType {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(FormatType::Exponent),
            "E" => Ok(FormatType::ExponentUppercase),
            "f" => Ok(FormatType::FixedPoint),
            "s" => Ok(FormatType::SI),
            "%" => Ok(FormatType::Percentage),
            "b" => Ok(FormatType::Binary),
            "o" => Ok(FormatType::Octal),
            "O" => Ok(FormatType::OctalUppercase),
            "d" => Ok(FormatType::Decimal),
            "x" => Ok(FormatType::Hex),
            "X" => Ok(FormatType::HexUppercase),
            _ => Err(FormatError::CouldNotParseFormatType),
        }
    }
}

impl<'a> From<&'a str> for FormatSpec<'a> {
    fn from(pattern: &'a str) -> FormatSpec<'a> {
        let re =
            Regex::new(r"^(?:(.)?([<>=^]))?([+\- ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?([A-Za-z%])?$")
                .unwrap();
        FormatSpec::from(re.captures(pattern).unwrap())
    }
}

impl<'a> From<Captures<'a>> for FormatSpec<'a> {
    /// Create a `FormatSpec` instance from a parsed format pattern string.
    fn from(c: Captures<'a>) -> Self {
        let align = c
            .get(2)
            .map(|s| s.as_str().parse().unwrap_or(Align::Right))
            .unwrap_or(Align::Right);
        let sign = match c.get(3).map(|m| m.as_str()) {
            Some("-") => Sign::OnlyNegative,
            Some("+") => Sign::Always,
            Some(" ") => Sign::SpaceOrDash,
            _ => Sign::OnlyNegative,
        };
        let type_prefix = matches!(c.get(4).map(|m| m.as_str()), Some("#"));
        let commas = matches!(c.get(7).map(|m| m.as_str()), Some(","));
        let precision = c
            .get(8)
            .map(|m| m.as_str()[1..].parse().unwrap())
            .or(Some(6));
        let format_type: FormatType = c
            .get(9)
            .and_then(|s| s.as_str().parse().ok())
            .unwrap_or(FormatType::None);

        let mut spec = Self {
            fill: c.get(1).map(|m| m.as_str()).or(Some(" ")),
            align,
            sign,
            type_prefix,
            zero_padding: c.get(5).is_some(),
            width: c.get(6).map(|m| m.as_str().parse().unwrap()).or(Some(0)),
            commas,
            precision,
            format_type,
        };

        // If zero fill is specified, padding goes after sign and before digits.
        if spec.zero_padding
            || (spec.fill.unwrap_or_default() == "0" && spec.align == Align::SignedRight)
        {
            spec.zero_padding = true;
            spec.fill = Some("0");
            spec.align = Align::SignedRight;
        }

        // Ignore precision for decimal notation.
        if spec.format_type == FormatType::Decimal {
            spec.precision = Some(0);
        };

        spec
    }
}
