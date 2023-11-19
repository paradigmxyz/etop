use regex::{Captures, Regex};
use std::cmp::{max, min};
use std::iter::repeat;

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

const PREFIXES: [&str; 17] = [
    "y", "z", "a", "f", "p", "n", "µ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
];
const DECIMAL_CHAR: char = '.';
const GROUP_DELIMITER_CHAR: char = ',';

/// A struct that defines the formatting specs and implements the formatting behavior.
///
/// Defines the characters used as a decimal symbol as well as the character used to
/// delimit groups of characters in the integer part of the number.
pub struct NumberFormat {
    decimal: char,
    group_delimiter: char,
}

/// Represents a destructured specification of a provided format pattern string.
#[derive(Debug)]
struct FormatSpec<'a> {
    zero: bool,
    fill: Option<&'a str>,
    align: Option<&'a str>,
    sign: Option<&'a str>,
    symbol: Option<&'a str>,
    width: Option<usize>,
    grouping: Option<&'a str>,
    precision: Option<i32>,
    format_type: Option<&'a str>,
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


impl NumberFormat {
    /// Create a new instance of HumanNumberFormat.
    pub fn new() -> Self {
        Self {
            decimal: '.',
            group_delimiter: ',',
        }
    }

    #[allow(dead_code)]
    fn get_significant_digits(input: &str) -> usize {
        let contains_dot = input.find(".").is_some();
        let mut dot_counted = false;
        let mut insignificant = 0;
        for char in input.chars() {
            match char {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => break,
                '.' => {
                    insignificant += 1;
                    dot_counted = true;
                }
                _ => insignificant += 1,
            }
        }

        if !contains_dot {
            for char in input.chars().rev() {
                match char {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => break,
                    _ => insignificant += 1,
                }
            }
        }

        input.len() - insignificant - (contains_dot && !dot_counted) as usize
    }

    /// Computes the decimal coefficient and exponent of the specified number `value` with supplied
    /// amount of significant digits. For example, decompose_to_coefficient_and_exponent(1.23, Option<2>)
    /// returns ("12", 0).
    fn decompose_to_coefficient_and_exponent(
        &self,
        value: f64,
        significant_digits: Option<usize>,
    ) -> (String, isize) {
        // Use exponential formatting to get the expected number of significant digits.
        let formatted_value = if significant_digits.is_some() {
            let precision = if significant_digits.unwrap() == 0 {
                0
            } else {
                significant_digits.unwrap() - 1
            };
            format!("{:.1$e}", value, precision)
        } else {
            format!("{:e}", value)
        };

        let exp_tokens: Vec<&str> = formatted_value.split('e').collect::<Vec<&str>>();
        let exponent = exp_tokens[1].parse().unwrap();

        // The `formatted_num` can have 2 shapes: `1e2` and `1.2e2`. Remove the decimal character
        // in case it's in the latter form.
        if exp_tokens[0].len() == 1 {
            (exp_tokens[0].to_owned(), exponent)
        } else {
            let dot_idx = exp_tokens[0]
                .chars()
                .position(|c| c == self.decimal)
                .unwrap();
            (
                format!(
                    "{}{}",
                    &exp_tokens[0][..dot_idx],
                    &exp_tokens[0][dot_idx + 1..]
                ),
                exponent,
            )
        }
    }

    /// Compute the [SI prefix](https://en.wikipedia.org/wiki/Metric_prefix) of the number and scale it accordingly.
    fn format_si_prefix(&self, value: f64, precision: Option<i32>) -> (String, isize) {
        let (coefficient, exponent) =
            self.decompose_to_coefficient_and_exponent(value, precision.map(|p| p as usize));
        let prefix_exponent = max(-8, min(8, (exponent as f32 / 3_f32).floor() as isize));
        let i: isize = exponent - prefix_exponent * 3 + 1;
        let n: isize = coefficient.len() as isize;

        if i == n {
            (coefficient, prefix_exponent)
        } else if i > n {
            (
                format!(
                    "{}{}",
                    coefficient,
                    repeat("0").take((i - n) as usize).collect::<String>()
                ),
                prefix_exponent,
            )
        } else if i > 0 {
            (
                format!(
                    "{}{}{}",
                    &coefficient[..i as usize],
                    self.decimal,
                    &coefficient[i as usize..]
                ),
                prefix_exponent,
            )
        } else {
            // less than 1 yocto
            (
                format!(
                    "0{}{}{}",
                    self.decimal,
                    repeat("0").take(i.abs() as usize).collect::<String>(),
                    self.decompose_to_coefficient_and_exponent(
                        value,
                        precision.and(Some(max(
                            0,
                            precision
                                .map(|p| (p - i.abs() as i32 - 1) as usize)
                                .unwrap()
                        )))
                    )
                    .0
                ),
                prefix_exponent,
            )
        }
    }

    /// Parse the formatting pattern and return a format specification based on the pattern.
    ///
    /// The parser is modeled after Python 3's [format specification mini-language](https://docs.python.org/3/library/string.html#format-specification-mini-language)
    /// [(PEP3101)](https://www.python.org/dev/peps/pep-3101/) with some minor implementation
    /// details changes.
    ///
    /// The format spec pattern is the following: [[fill]align][sign][symbol][0][width][,][.precision][type]
    fn parse_pattern<'a>(&self, pattern: &'a str) -> FormatSpec<'a> {
        let re =
            Regex::new(r"^(?:(.)?([<>=^]))?([+\- ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?([A-Za-z%])?$")
                .unwrap();
        FormatSpec::from(re.captures(pattern).unwrap())
    }

    /// Group digits using the `group_delimiter` character.
    ///
    /// A width is going to be specified (>0) only when the formatted value should be filled in
    /// with "0" characters before the number itself (e.g. using a "020f" or "0=12f" pattern).
    ///
    /// If width > 0, the result will fit into the provided width.
    /// In case the width > 0 and the grouped value starts with the grouping character
    /// (e.g. width = 4, value = 0001 -> ,001), it will be formatted as 0,001, since ,001
    /// is not a valid representation.
    ///
    /// If width = 0, the result will group all passed digits without truncating any of them.
    fn group_value(&self, value: &str, width: usize) -> String {
        let mut reversed_chars: Vec<&[char]> = Vec::new();
        let input_chars: Vec<char> = value.chars().rev().collect();
        let separator: [char; 1] = [self.group_delimiter];

        // After the below loop, an input of "1234" is going to be
        // transformed into `vec![['4', '3', '2'], [','], ['1'], [',']]`.
        for group in input_chars.chunks(3) {
            reversed_chars.push(group);
            reversed_chars.push(&separator);
        }
        // pop last grouping character since it is going to become the leading one after reverse.
        reversed_chars.pop();

        // Flatten the reversed_chars vec
        let grouped: Vec<&char> = reversed_chars.into_iter().flatten().collect();

        // Assure the grouped value fits into provided width in case width > 0
        if width > 0 && grouped.len() > width {
            // If the first character is going to be the group delimiter,
            // keep the one preceding the group delimiter.
            let to_skip = if grouped[width - 1] == &separator[0] {
                grouped.len() - width - 1
            } else {
                grouped.len() - width
            };
            grouped.into_iter().rev().skip(to_skip).collect::<String>()
        } else {
            grouped.into_iter().rev().collect::<String>()
        }
    }

    /// Format the number using scientific notation. The exponent is always represented with
    /// the corresponding sign and at least 2 digits (e.g. 1e+01, 2.1e-02, 42.12e+210).
    ///
    /// The `format_type` is either a small "e" or a capital "E". Also, the format spec pattern
    /// might require displaying a decimal point even if the formatted number does not contain
    /// any decimal digits.
    fn get_formatted_exp_value(
        &self,
        format_type: &str,
        value: f64,
        precision: usize,
        include_decimal_point: bool,
    ) -> String {
        let formatted = format!("{:.1$e}", value, precision);
        let tokens = formatted.split(format_type).collect::<Vec<&str>>();

        let exp_suffix = if &tokens[1][0..1] == "-" {
            if tokens[1].len() == 2 {
                format!("-0{}", &tokens[1][1..])
            } else {
                tokens[1].to_owned()
            }
        } else {
            format!("+{:0>2}", &tokens[1])
        };

        let possible_decimal = if include_decimal_point && precision == 0 {
            format_args!("{}", self.decimal).to_string()
        } else {
            "".to_owned()
        };

        format!(
            "{}{}{}{}",
            &tokens[0], possible_decimal, format_type, exp_suffix
        )
    }

    /// Compute the sign prefix to display based on num sign and format spec.
    ///
    /// If the number is negative, always show "-" sign.
    /// Otherwise, if the format spec contains:
    ///   - "+" sign, show a "+" sign for positive numbers
    ///   - " " a blank space, leave a blank space for positive numbers
    ///
    /// If the format_spec does not contain any info regarding the sign, use an empty string.
    fn get_sign_prefix(&self, is_negative: bool, format_spec: &FormatSpec) -> &str {
        if is_negative {
            "-"
        } else if format_spec.sign.unwrap() == "+" {
            "+"
        } else if format_spec.sign.unwrap() == " " {
            " "
        } else {
            ""
        }
    }

    /// Format a number to a specific human readable form defined by the format spec pattern.
    /// The method takes in a string specifier and a number and returns the string representation
    /// of the formatted number.
    pub fn format<T: Into<f64>>(&self, pattern: &str, input: T) -> String {
        let format_spec = self.parse_pattern(pattern);

        let input_f64: f64 = input.into();
        let mut value_is_negative: bool = input_f64.is_sign_negative();

        let mut decimal_part = String::new();
        let mut si_prefix_exponent: &str = "";
        let unit_of_measurement: &str = match format_spec.format_type {
            Some("%") => "%",
            _ => "",
        };

        let mut value = match format_spec.format_type {
            Some("%") => format!(
                "{:.1$}",
                input_f64.abs() * 100_f64,
                format_spec.precision.unwrap() as usize
            ),
            Some("b") => format!("{:#b}", input_f64.abs() as i64)[2..].into(),
            Some("o") | Some("O") => format!("{:#o}", input_f64.abs() as i64)[2..].into(),
            Some("x") => format!("{:#x}", input_f64.abs() as i64)[2..].into(),
            Some("X") => format!("{:#X}", input_f64.abs() as i64)[2..].into(),
            Some("f") if format_spec.symbol.unwrap_or_default() == "#" => {
                let maybe_decimal = if format_spec.precision.unwrap() == 0 {
                    self.decimal.to_string()
                } else {
                    "".to_string()
                };
                format!(
                    "{:.2$}{}",
                    input_f64.abs(),
                    maybe_decimal,
                    format_spec.precision.unwrap() as usize
                )
            }
            Some("e") => self.get_formatted_exp_value(
                "e",
                input_f64.abs(),
                format_spec.precision.unwrap() as usize,
                format_spec.symbol.unwrap_or_default() == "#",
            ),
            Some("E") => self.get_formatted_exp_value(
                "E",
                input_f64.abs(),
                format_spec.precision.unwrap() as usize,
                format_spec.symbol.unwrap_or_default() == "#",
            ),
            Some("s") => {
                let (val, si_prefix) =
                    self.format_si_prefix(input_f64.abs(), format_spec.precision);
                si_prefix_exponent = PREFIXES[(8 + si_prefix) as usize];
                val
            }
            _ => format!(
                "{:.1$}",
                input_f64.abs(),
                format_spec.precision.unwrap() as usize
            ),
        };

        // If a negative value rounds to zero after formatting, and no explicit positive sign is requested, hide the sign.
        if format_spec.format_type != Some("x")
            && format_spec.format_type != Some("X")
            && value_is_negative
            && value.parse::<f64>().unwrap() == 0_f64
            && format_spec.sign.unwrap_or("+") != "+"
        {
            value_is_negative = false;
        }

        let sign_prefix = self.get_sign_prefix(value_is_negative, &format_spec);

        let leading_part = match format_spec.symbol {
            Some("#") => match format_spec.format_type {
                Some("b") => "0b",
                Some("o") => "0o",
                Some("x") => "0x",
                Some("O") => "0O",
                Some("X") => "0x",
                _ => "",
            },
            _ => "",
        };

        // Split the integer part of the value for grouping purposes and attach the decimal part as suffix.
        let mut chars = value.chars().enumerate().into_iter();
        while let Some((i, c)) = chars.next() {
            if !"0123456789".find(c).is_some() {
                decimal_part = value[i..].to_owned();
                value = value[..i].to_owned();
                break;
            }
        }

        // Compute the prefix and suffix.
        let prefix = format!("{}{}", sign_prefix, leading_part);
        let suffix = format!(
            "{}{}{}",
            decimal_part, si_prefix_exponent, unit_of_measurement
        );

        // If should group and filling character is different than "0",
        // group digits before applying padding.
        if format_spec.grouping.is_some() && !format_spec.zero {
            value = self.group_value(&value, 0)
        }

        // Compute the padding.
        let length = prefix.len() + value.to_string().len() + suffix.len();
        let mut padding = if length < format_spec.width.unwrap() {
            vec![format_spec.fill.unwrap(); format_spec.width.unwrap() - length].join("")
        } else {
            "".to_owned()
        };

        // If "0" is the filling character, grouping is applied after computing padding.
        if format_spec.grouping.is_some() && format_spec.zero {
            value = self.group_value(
                format!("{}{}", &padding, value).as_str(),
                if padding.len() > 0 {
                    format_spec.width.unwrap() - suffix.len()
                } else {
                    0
                },
            );
            padding = "".to_owned();
        };

        match format_spec.align {
            Some("<") => format!("{}{}{}{}", prefix, value, suffix, padding),
            Some("=") => format!("{}{}{}{}", prefix, padding, value, suffix),
            Some("^") => format!(
                "{}{}{}{}{}",
                &padding[..padding.len() / 2],
                prefix,
                value,
                suffix,
                &padding[padding.len() / 2..]
            ),
            _ => format!("{}{}{}{}", padding, prefix, value, suffix),
        }
    }
}

