use super::types::{FormatError, Sign, DECIMAL_CHAR, GROUP_DELIMITER_CHAR};
use std::cmp::{max, min};

#[allow(dead_code)]
pub(crate) fn get_significant_digits(input: &str) -> usize {
    let contains_dot = input.contains('.');
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
pub(crate) fn decompose_to_coefficient_and_exponent(
    value: f64,
    significant_digits: Option<usize>,
) -> Result<(String, isize), FormatError> {
    // Use exponential formatting to get the expected number of significant digits.
    let formatted_value = if let Some(significant_digits) = significant_digits {
        let precision = if significant_digits == 0 {
            0
        } else {
            significant_digits - 1
        };
        format!("{:.1$e}", value, precision)
    } else {
        format!("{:e}", value)
    };

    let exp_tokens: Vec<&str> = formatted_value.split('e').collect::<Vec<&str>>();
    let exponent = exp_tokens[1]
        .parse()
        .map_err(|_| FormatError::CouldNotDecomposeCoefficientExponent)?;

    // The `formatted_num` can have 2 shapes: `1e2` and `1.2e2`. Remove the decimal character
    // in case it's in the latter form.
    if exp_tokens[0].len() == 1 {
        Ok((exp_tokens[0].to_owned(), exponent))
    } else {
        let dot_idx = exp_tokens[0]
            .chars()
            .position(|c| c == DECIMAL_CHAR)
            .ok_or(FormatError::CouldNotDecomposeCoefficientExponent)?;
        let coefficient = format!(
            "{}{}",
            &exp_tokens[0][..dot_idx],
            &exp_tokens[0][dot_idx + 1..]
        );
        Ok((coefficient, exponent))
    }
}

/// Compute the [SI prefix](https://en.wikipedia.org/wiki/Metric_prefix) of the number and scale it accordingly.
pub(crate) fn format_si_prefix(
    value: f64,
    precision: Option<usize>,
) -> Result<(String, isize), FormatError> {
    let (coefficient, exponent) = decompose_to_coefficient_and_exponent(value, precision)?;
    let prefix_exponent = max(-8, min(8, (exponent as f32 / 3_f32).floor() as isize));
    let i: isize = exponent - prefix_exponent * 3 + 1;
    let n: isize = coefficient.len() as isize;

    if i == n {
        Ok((coefficient, prefix_exponent))
    } else if i > n {
        let coefficient = format!("{}{}", coefficient, "0".repeat((i - n) as usize),);
        Ok((coefficient, prefix_exponent))
    } else if i > 0 {
        let coefficient = format!(
            "{}{}{}",
            &coefficient[..i as usize],
            DECIMAL_CHAR,
            &coefficient[i as usize..]
        );
        Ok((coefficient, prefix_exponent))
    } else {
        // less than 1 yocto
        let inner_precision = precision
            .map(|p| (p as i32 - i.abs() as i32 - 1) as usize)
            .ok_or(FormatError::CouldNotDecomposeCoefficientExponent)?;
        let coefficient = decompose_to_coefficient_and_exponent(
            value,
            precision.and(Some(max(0, inner_precision))),
        )?
        .0;
        let coefficient = format!(
            "0{}{}{}",
            DECIMAL_CHAR,
            "0".repeat(i.unsigned_abs()),
            coefficient
        );
        Ok((coefficient, prefix_exponent))
    }
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
pub(crate) fn group_value(value: &str, width: usize) -> String {
    let mut reversed_chars: Vec<&[char]> = Vec::new();
    let input_chars: Vec<char> = value.chars().rev().collect();
    let separator: [char; 1] = [GROUP_DELIMITER_CHAR];

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
pub(crate) fn get_formatted_exp_value(
    format_type: &str,
    value: f64,
    precision: usize,
    include_decimal_point: bool,
) -> String {
    let formatted = format!("{:.1$e}", value, precision);
    let tokens = formatted.split('e').collect::<Vec<&str>>();
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
        format_args!("{}", DECIMAL_CHAR).to_string()
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
pub(crate) fn get_sign_prefix(is_negative: bool, format_sign: &Sign) -> &'static str {
    if is_negative {
        "-"
    } else if let Sign::Always = format_sign {
        "+"
    } else if let Sign::SpaceOrDash = format_sign {
        " "
    } else {
        ""
    }
}
