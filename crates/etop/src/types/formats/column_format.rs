use super::cell_format::CellFormat;
use super::unknown_format::UnknownFormat;
use crate::EtopError;
use polars::prelude::*;
use toolstr::{BinaryFormat, BoolFormat, NumberFormat, StringFormat};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ColumnFormat {
    pub name: String,
    pub display_name: String,
    pub format: CellFormat,
}

impl Default for ColumnFormat {
    fn default() -> ColumnFormat {
        let format = UnknownFormat {
            min_width: None,
            max_width: None,
        };
        ColumnFormat {
            name: "".to_string(),
            display_name: "".to_string(),
            format: CellFormat::Unknown(format),
        }
    }
}

impl ColumnFormat {
    pub fn header_width(&self) -> usize {
        self.display_name
            .split('\n')
            .map(|s| s.len())
            .max()
            .unwrap_or(0)
    }

    pub fn get_min_width(&self) -> usize {
        self.format.get_min_width().unwrap_or(0)
    }

    pub fn get_max_width(&self) -> usize {
        self.format.get_max_width().unwrap_or(usize::MAX)
    }

    pub fn format(&self, series: &Series) -> Result<Vec<String>, EtopError> {
        let formatted: Result<Vec<String>, toolstr::FormatError> = match series.dtype() {
            DataType::Binary => {
                let fmt: BinaryFormat = self.format.clone().try_into()?;
                series
                    .binary()?
                    .into_iter()
                    .map(|v| fmt.format_option(v, ""))
                    .collect()
            }
            DataType::Utf8 => {
                let fmt: StringFormat = self.format.clone().try_into()?;
                series
                    .utf8()?
                    .into_iter()
                    .map(|v| fmt.format_option(v, ""))
                    .collect()
            }
            dtype if dtype.is_numeric() => {
                let fmt: NumberFormat = self.format.clone().try_into()?;
                series
                    .to_float()?
                    .f64()?
                    .into_iter()
                    .map(|v| fmt.format_option(v, ""))
                    .collect()
            }
            DataType::Boolean => {
                let fmt: BoolFormat = self.format.clone().try_into()?;
                series
                    .bool()?
                    .into_iter()
                    .map(|v| fmt.format_option(v, ""))
                    .collect()
            }
            dtype => {
                let message = format!("column {} has type {}", series.name(), dtype);
                return Err(EtopError::UnsupportedDatatype(message));
            }
        };
        formatted.map_err(EtopError::FormatError)
    }
}

// builder
impl ColumnFormat {
    pub fn new() -> ColumnFormat {
        ColumnFormat::default()
    }

    pub fn name<T: AsRef<str>>(mut self, name: T) -> ColumnFormat {
        let name = name.as_ref().to_string();
        self.name = name.clone();
        if self.display_name.is_empty() {
            self.display_name = name
        };
        self
    }

    pub fn display_name<T: AsRef<str>>(mut self, display_name: T) -> ColumnFormat {
        self.display_name = display_name.as_ref().to_string();
        self
    }

    pub fn newline_underscores(mut self) -> ColumnFormat {
        self.display_name = self.display_name.replace('_', "\n");
        self
    }

    pub fn width(self, width: usize) -> ColumnFormat {
        self.min_width(width).max_width(width)
    }

    pub fn min_width(mut self, width: usize) -> ColumnFormat {
        self.format = self.format.min_width(width);
        self
    }

    pub fn max_width(mut self, width: usize) -> ColumnFormat {
        self.format = self.format.max_width(width);
        self
    }
}
