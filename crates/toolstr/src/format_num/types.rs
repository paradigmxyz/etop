use regex::Captures;
use regex::Regex;

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
    pub zero: bool,
    pub fill: Option<&'a str>,
    pub align: Option<&'a str>,
    pub sign: Option<&'a str>,
    pub symbol: Option<&'a str>,
    pub width: Option<usize>,
    pub grouping: Option<&'a str>,
    pub precision: Option<i32>,
    pub format_type: Option<&'a str>,
}

impl<'a> From<&'a str> for FormatSpec<'a> {
    fn from(pattern: &'a str) -> FormatSpec<'a> {
        let re = Regex::new(r"^(?:(.)?([<>=^]))?([+\- ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?([A-Za-z%])?$")
            .unwrap();
        FormatSpec::from(re.captures(pattern).unwrap())
    }
}

impl<'a> From<Captures<'a>> for FormatSpec<'a> {
    /// Create a `FormatSpec` instance from a parsed format pattern string.
    fn from(c: Captures<'a>) -> Self {
        let mut spec = Self {
            fill: c.get(1).map(|m| m.as_str()).or(Some(" ")),
            align: c.get(2).map(|m| m.as_str()),
            sign: c.get(3).map(|m| m.as_str()).or(Some("-")),
            symbol: c.get(4).map(|m| m.as_str()),
            zero: c.get(5).is_some(),
            width: c.get(6).map(|m| m.as_str().parse().unwrap()).or(Some(0)),
            grouping: c.get(7).map(|m| m.as_str()),
            precision: c
                .get(8)
                .map(|m| m.as_str()[1..].parse().unwrap())
                .or(Some(6)),
            format_type: c.get(9).map(|m| m.as_str()),
        };

        // If zero fill is specified, padding goes after sign and before digits.
        if spec.zero
            || (spec.fill.unwrap_or_default() == "0" && spec.align.unwrap_or_default() == "=")
        {
            spec.zero = true;
            spec.fill = Some("0");
            spec.align = Some("=");
        }

        // Ignore precision for decimal notation.
        if spec.format_type.unwrap_or_default() == "d" {
            spec.precision = Some(0);
        };

        spec
    }
}
