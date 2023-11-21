use super::types::{Align, FormatError, FormatType, NumberFormat, Sign, DEFAULT_PRECISION};
use regex::{Captures, Regex};
use std::fmt;
use std::str::FromStr;

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            FormatError::CouldNotParseFormatType => "could not parse format type",
            FormatError::CouldNotDecomposeCoefficientExponent => {
                "could not deomponse coefficient exponent"
            }
            FormatError::CouldNotCreateRegex => "could not create regex",
            FormatError::RegexCouldNotMatch => "regex could not match",
        };
        write!(f, "{}", message)
    }
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

impl TryFrom<&str> for NumberFormat {
    type Error = FormatError;

    fn try_from(pattern: &str) -> Result<NumberFormat, FormatError> {
        let re =
            Regex::new(r"^(?:(.)?([<>=^]))?([+\- ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?([A-Za-z%])?$")
                .map_err(|_| FormatError::CouldNotCreateRegex)?;
        let captures = re
            .captures(pattern)
            .ok_or(FormatError::RegexCouldNotMatch)?;
        Ok(NumberFormat::from(captures))
    }
}

impl From<Captures<'_>> for NumberFormat {
    /// Create a `NumberFormat` instance from a parsed format pattern string.
    fn from(c: Captures<'_>) -> Self {
        let fill = c
            .get(1)
            .and_then(|m| m.as_str().chars().next())
            .unwrap_or(' ');
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
        let zero_padding = c.get(5).is_some();
        let width = c
            .get(6)
            .map(|m| m.as_str().parse().unwrap_or(0))
            .unwrap_or(0);
        let commas = matches!(c.get(7).map(|m| m.as_str()), Some(","));
        let precision = c
            .get(8)
            .map(|m| {
                m.as_str()
                    .get(1..)
                    .unwrap_or_default()
                    .parse()
                    .unwrap_or(DEFAULT_PRECISION)
            })
            .unwrap_or(DEFAULT_PRECISION);
        let format_type: FormatType = c
            .get(9)
            .and_then(|s| s.as_str().parse().ok())
            .unwrap_or(FormatType::None);

        let mut spec = Self {
            fill,
            align,
            sign,
            type_prefix,
            zero_padding,
            width,
            commas,
            precision,
            format_type,
        };

        // If zero fill is specified, padding goes after sign and before digits.
        if spec.zero_padding || (spec.fill == '0' && spec.align == Align::SignedRight) {
            spec.zero_padding = true;
            spec.fill = '0';
            spec.align = Align::SignedRight;
        }

        // Ignore precision for decimal notation.
        if spec.format_type == FormatType::Decimal {
            spec.precision = 0;
        };

        spec
    }
}
