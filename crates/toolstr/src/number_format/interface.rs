use super::process;
use super::types::{Align, FormatError, FormatType, NumberFormat, Sign, DECIMAL_CHAR, PREFIXES};

/// Format a number to a specific human readable form defined by the format spec pattern.
/// The method takes in a string specifier and a number and returns the string representation
/// of the formatted number.
pub fn format<P: TryInto<NumberFormat, Error = FormatError>, T: Into<f64>>(
    pattern: P,
    input: T,
) -> Result<String, FormatError> {
    number_format(&pattern.try_into()?, input)
}

pub(crate) fn number_format<T: Into<f64>>(
    number_format: &NumberFormat,
    input: T,
) -> Result<String, FormatError> {
    let input_f64: f64 = input.into();
    let mut value_is_negative: bool = input_f64.is_sign_negative();

    let mut decimal_part = String::new();
    let mut si_prefix_exponent: &str = "";
    let unit_of_measurement: &str = match number_format.format_type {
        FormatType::Percentage => "%",
        _ => "",
    };

    let mut value = match number_format.format_type {
        FormatType::Percentage => {
            format!("{:.1$}", input_f64.abs() * 100_f64, number_format.precision)
        }
        FormatType::Binary => format!("{:#b}", input_f64.abs() as i64)[2..].into(),
        FormatType::Octal | FormatType::OctalUppercase => {
            format!("{:#o}", input_f64.abs() as i64)[2..].into()
        }
        FormatType::Hex => format!("{:#x}", input_f64.abs() as i64)[2..].into(),
        FormatType::HexUppercase => format!("{:#X}", input_f64.abs() as i64)[2..].into(),
        FormatType::FixedPoint if number_format.type_prefix => {
            let maybe_decimal = if number_format.precision == 0 {
                DECIMAL_CHAR.to_string()
            } else {
                "".to_string()
            };
            format!(
                "{:.2$}{}",
                input_f64.abs(),
                maybe_decimal,
                number_format.precision
            )
        }
        FormatType::Exponent => process::get_formatted_exp_value(
            "e",
            input_f64.abs(),
            number_format.precision,
            number_format.type_prefix,
        ),
        FormatType::ExponentUppercase => process::get_formatted_exp_value(
            "E",
            input_f64.abs(),
            number_format.precision,
            number_format.type_prefix,
        ),
        FormatType::SI => {
            let (val, si_prefix) =
                process::format_si_prefix(input_f64.abs(), Some(number_format.precision))?;
            si_prefix_exponent = PREFIXES[(8 + si_prefix) as usize];
            val
        }
        _ => format!("{:.1$}", input_f64.abs(), number_format.precision),
    };

    // If a negative value rounds to zero after formatting, and no explicit positive sign is requested, hide the sign.
    if number_format.format_type != FormatType::Hex
        && number_format.format_type != FormatType::HexUppercase
        && value_is_negative
        && value.parse::<f64>() == Ok(0_f64)
        && (number_format.sign != Sign::Always)
    {
        value_is_negative = false;
    }

    let sign_prefix = process::get_sign_prefix(value_is_negative, &number_format.sign);

    let leading_part = match number_format.type_prefix {
        true => match number_format.format_type {
            FormatType::Binary => "0b",
            FormatType::Octal => "0o",
            FormatType::Hex => "0x",
            FormatType::OctalUppercase => "0O",
            FormatType::HexUppercase => "0x",
            _ => "",
        },
        false => "",
    };

    // Split the integer part of the value for grouping purposes and attach the decimal part as suffix.
    for (i, c) in value.chars().enumerate() {
        if "0123456789".find(c).is_none() {
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
    if number_format.commas && !number_format.zero_padding {
        value = process::group_value(&value, 0)
    }

    // Compute the padding.
    let length = prefix.len() + value.to_string().len() + suffix.len();
    let mut padding = if length < number_format.width {
        vec![number_format.fill.to_string(); number_format.width - length].join("")
    } else {
        "".to_owned()
    };

    // If "0" is the filling character, grouping is applied after computing padding.
    if number_format.commas && number_format.zero_padding {
        value = process::group_value(
            format!("{}{}", &padding, value).as_str(),
            if !padding.is_empty() {
                number_format.width - suffix.len()
            } else {
                0
            },
        );
        padding = "".to_owned();
    };

    let formatted = match number_format.align {
        Align::Left => format!("{}{}{}{}", prefix, value, suffix, padding),
        Align::SignedRight => format!("{}{}{}{}", prefix, padding, value, suffix),
        Align::Right => format!("{}{}{}{}", padding, prefix, value, suffix),
        Align::Center => format!(
            "{}{}{}{}{}",
            &padding[..padding.len() / 2],
            prefix,
            value,
            suffix,
            &padding[padding.len() / 2..]
        ),
    };

    Ok(formatted)
}
