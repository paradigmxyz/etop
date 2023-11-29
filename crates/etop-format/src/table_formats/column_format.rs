use crate::{
    BinaryFormat, BoolFormat, CellFormat, CellFormatShorthand, FormatError, NumberFormat,
    StringFormat, UnknownFormat,
};
use polars::prelude::*;
use unicode_truncate::{Alignment, UnicodeTruncateStr};

/// column format shorthand
#[derive(Debug, Clone)]
pub struct ColumnFormatShorthand {
    /// name
    pub name: String,
    /// display name
    pub display_name: String,
    /// format
    pub format: CellFormatShorthand,
    /// alignment
    pub align: ColumnAlign,
}

impl ColumnFormatShorthand {
    /// finalize shorthand into format format
    pub fn finalize(self, dtype: &DataType) -> Result<ColumnFormat, FormatError> {
        Ok(ColumnFormat {
            name: self.name,
            display_name: self.display_name,
            format: self.format.finalize(dtype)?,
            align: self.align,
        })
    }
}

impl Default for ColumnFormatShorthand {
    fn default() -> ColumnFormatShorthand {
        let format = UnknownFormat { min_width: None, max_width: None };
        ColumnFormatShorthand {
            name: "".to_string(),
            display_name: "".to_string(),
            format: CellFormatShorthand::Unknown(format),
            align: ColumnAlign::Right,
        }
    }
}

/// column format
#[derive(Debug, Clone)]
pub struct ColumnFormat {
    /// name
    pub name: String,
    /// display name
    pub display_name: String,
    /// format
    pub format: CellFormat,
    /// alignment
    pub align: ColumnAlign,
}

/// column alignment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColumnAlign {
    /// left
    Left,
    /// right
    Right,
}

impl ColumnFormat {
    /// get header width
    pub fn header_width(&self) -> usize {
        self.display_name
            .split('\n')
            // .map(unicode_width::UnicodeWidthStr::width)
            // .map(unicode_width::UnicodeWidthStr::width_cjk)
            .map(|s| s.chars().count())
            .max()
            .unwrap_or(0)
    }

    /// get min width
    pub fn get_min_width(&self) -> usize {
        self.format.get_min_width().unwrap_or(0)
    }

    /// get max width
    pub fn get_max_width(&self) -> usize {
        self.format.get_max_width().unwrap_or(usize::MAX)
    }

    /// format series
    pub fn format(&self, series: &Series) -> Result<Vec<String>, FormatError> {
        let formatted: Result<Vec<String>, FormatError> = match series.dtype() {
            DataType::Binary => {
                let fmt: BinaryFormat = self.format.clone().try_into()?;
                series.binary()?.into_iter().map(|v| fmt.format_option(v, "")).collect()
            }
            DataType::Utf8 => {
                let fmt: StringFormat = self.format.clone().try_into()?;
                series.utf8()?.into_iter().map(|v| fmt.format_option(v, "")).collect()
            }
            dtype if dtype.is_numeric() => {
                let fmt: NumberFormat = self.format.clone().try_into()?;
                series.to_float()?.f64()?.into_iter().map(|v| fmt.format_option(v, "")).collect()
            }
            DataType::Boolean => {
                let fmt: BoolFormat = self.format.clone().try_into()?;
                series.bool()?.into_iter().map(|v| fmt.format_option(v, "")).collect()
            }
            dtype => {
                let message = format!("column {} has type {}", series.name(), dtype);
                return Err(FormatError::UnsupportedDatatype(message));
            }
        };
        let formatted = formatted?;

        let max_width = formatted
            .iter()
            // .map(|s| unicode_width::UnicodeWidthStr::width(s.as_str()))
            // .map(|s| unicode_width::UnicodeWidthStr::width_cjk(s.as_str()))
            .map(|s| s.chars().count())
            .max()
            .unwrap_or(0);

        let formatted = if self.align == ColumnAlign::Right {
            formatted
                .into_iter()
                .map(|s| s.unicode_pad(max_width, Alignment::Right, true).to_string())
                .collect()
        } else {
            formatted
                .into_iter()
                .map(|s| s.unicode_pad(max_width, Alignment::Left, true).to_string())
                .collect()
        };

        Ok(formatted)
    }
}

// builder
impl ColumnFormat {
    /// set name
    pub fn name<T: AsRef<str>>(mut self, name: T) -> ColumnFormat {
        let name = name.as_ref().to_string();
        self.name = name.clone();
        if self.display_name.is_empty() {
            self.display_name = name
        };
        self
    }

    /// set display name
    pub fn display_name<T: AsRef<str>>(mut self, display_name: T) -> ColumnFormat {
        self.display_name = display_name.as_ref().to_string();
        self
    }

    /// set newline underscores
    pub fn newline_underscores(mut self) -> ColumnFormat {
        self.display_name = self.display_name.replace('_', "\n");
        self
    }

    /// set width
    pub fn width(self, width: usize) -> ColumnFormat {
        self.min_width(width).max_width(width)
    }

    /// set min width
    pub fn min_width(mut self, width: usize) -> ColumnFormat {
        self.format = self.format.min_width(width);
        self
    }

    /// set max width
    pub fn max_width(mut self, width: usize) -> ColumnFormat {
        self.format = self.format.max_width(width);
        self
    }
}

// builder
impl ColumnFormatShorthand {
    /// new
    pub fn new() -> ColumnFormatShorthand {
        ColumnFormatShorthand::default()
    }

    /// set name
    pub fn name<T: AsRef<str>>(mut self, name: T) -> ColumnFormatShorthand {
        let name = name.as_ref().to_string();
        self.name = name.clone();
        if self.display_name.is_empty() {
            self.display_name = name
        };
        self
    }

    /// set display name
    pub fn display_name<T: AsRef<str>>(mut self, display_name: T) -> ColumnFormatShorthand {
        self.display_name = display_name.as_ref().to_string();
        self
    }

    /// set newline underscores
    pub fn newline_underscores(mut self) -> ColumnFormatShorthand {
        self.display_name = self.display_name.replace('_', "\n");
        self
    }

    /// set width
    pub fn width(self, width: usize) -> ColumnFormatShorthand {
        self.min_width(width).max_width(width)
    }

    /// set min width
    pub fn min_width(mut self, width: usize) -> ColumnFormatShorthand {
        self.format = self.format.min_width(width);
        self
    }

    /// set max width
    pub fn max_width(mut self, width: usize) -> ColumnFormatShorthand {
        self.format = self.format.max_width(width);
        self
    }

    /// set format
    pub fn set_format<T: Into<CellFormatShorthand>>(mut self, format: T) -> ColumnFormatShorthand {
        self.format = format.into();
        self
    }
}
