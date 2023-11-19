use super::types::NumberFormat;

pub fn format_num<T: Into<f64>>(pattern: &str, input: T) -> String {
    NumberFormat::new().format(pattern, input)
}
