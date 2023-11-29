use crate::{
    BinaryFormat, BoolFormat, FormatError, FormatType, NumberFormat, StringFormat, UnknownFormat,
};
use polars::prelude::DataType;

/// cell format shorthand
#[derive(Debug, Clone)]
pub enum CellFormatShorthand {
    /// number format
    Number(NumberFormat),
    /// binary format
    Binary(BinaryFormat),
    /// string format
    String(StringFormat),
    /// bool format
    Bool(BoolFormat),
    /// unknown format
    Unknown(UnknownFormat),
}

impl From<NumberFormat> for CellFormatShorthand {
    fn from(format: NumberFormat) -> CellFormatShorthand {
        CellFormatShorthand::Number(format)
    }
}

impl From<StringFormat> for CellFormatShorthand {
    fn from(format: StringFormat) -> CellFormatShorthand {
        CellFormatShorthand::String(format)
    }
}

impl From<BinaryFormat> for CellFormatShorthand {
    fn from(format: BinaryFormat) -> CellFormatShorthand {
        CellFormatShorthand::Binary(format)
    }
}

impl From<BoolFormat> for CellFormatShorthand {
    fn from(format: BoolFormat) -> CellFormatShorthand {
        CellFormatShorthand::Bool(format)
    }
}

impl CellFormatShorthand {
    /// set min width
    pub fn min_width(self, min_width: usize) -> CellFormatShorthand {
        match self {
            CellFormatShorthand::Number(fmt) => {
                CellFormatShorthand::Number(fmt.min_width(min_width))
            }
            CellFormatShorthand::String(fmt) => {
                CellFormatShorthand::String(fmt.min_width(min_width))
            }
            CellFormatShorthand::Binary(fmt) => {
                CellFormatShorthand::Binary(fmt.min_width(min_width))
            }
            CellFormatShorthand::Bool(fmt) => CellFormatShorthand::Bool(fmt.min_width(min_width)),
            CellFormatShorthand::Unknown(fmt) => {
                CellFormatShorthand::Unknown(fmt.min_width(min_width))
            }
        }
    }

    /// set max width
    pub fn max_width(self, max_width: usize) -> CellFormatShorthand {
        match self {
            CellFormatShorthand::Number(fmt) => {
                CellFormatShorthand::Number(fmt.max_width(max_width))
            }
            CellFormatShorthand::String(fmt) => {
                CellFormatShorthand::String(fmt.max_width(max_width))
            }
            CellFormatShorthand::Binary(fmt) => {
                CellFormatShorthand::Binary(fmt.max_width(max_width))
            }
            CellFormatShorthand::Bool(fmt) => CellFormatShorthand::Bool(fmt.max_width(max_width)),
            CellFormatShorthand::Unknown(fmt) => {
                CellFormatShorthand::Unknown(fmt.max_width(max_width))
            }
        }
    }

    /// convert shorthand into formal version
    pub fn finalize(self, dtype: &DataType) -> Result<CellFormat, FormatError> {
        let fmt = match self {
            CellFormatShorthand::Number(fmt) => CellFormat::Number(fmt),
            CellFormatShorthand::Binary(fmt) => CellFormat::Binary(fmt),
            CellFormatShorthand::String(fmt) => CellFormat::String(fmt),
            CellFormatShorthand::Bool(fmt) => CellFormat::Bool(fmt),
            CellFormatShorthand::Unknown(fmt) => match dtype {
                DataType::Utf8 => CellFormat::String(fmt.into()),
                DataType::Boolean => CellFormat::Bool(fmt.into()),
                DataType::Binary => CellFormat::Binary(fmt.into()),
                dtype if dtype.is_integer() => {
                    let fmt: NumberFormat = fmt.into();
                    let fmt = fmt.format_type(&FormatType::Decimal).precision(0);
                    CellFormat::Number(fmt)
                }
                dtype if dtype.is_float() => {
                    let fmt: NumberFormat = fmt.into();
                    let fmt = fmt.format_type(&FormatType::Exponent);
                    CellFormat::Number(fmt)
                }
                _ => {
                    return Err(FormatError::UnsupportedDatatype(format!(
                        "Unsupported datatype: {:?}",
                        dtype
                    )))
                }
            },
        };
        Ok(fmt)
    }
}

/// cell format
#[derive(Debug, Clone)]
pub enum CellFormat {
    /// number format
    Number(NumberFormat),
    /// binary format
    Binary(BinaryFormat),
    /// string format
    String(StringFormat),
    /// bool format
    Bool(BoolFormat),
}

impl CellFormat {
    /// set min width
    pub fn min_width(self, min_width: usize) -> CellFormat {
        match self {
            CellFormat::Number(fmt) => CellFormat::Number(fmt.min_width(min_width)),
            CellFormat::String(fmt) => CellFormat::String(fmt.min_width(min_width)),
            CellFormat::Binary(fmt) => CellFormat::Binary(fmt.min_width(min_width)),
            CellFormat::Bool(fmt) => CellFormat::Bool(fmt.min_width(min_width)),
        }
    }

    /// set max width
    pub fn max_width(self, max_width: usize) -> CellFormat {
        match self {
            CellFormat::Number(fmt) => CellFormat::Number(fmt.max_width(max_width)),
            CellFormat::String(fmt) => CellFormat::String(fmt.max_width(max_width)),
            CellFormat::Binary(fmt) => CellFormat::Binary(fmt.max_width(max_width)),
            CellFormat::Bool(fmt) => CellFormat::Bool(fmt.max_width(max_width)),
        }
    }

    /// get min width
    pub fn get_min_width(&self) -> Option<usize> {
        match self {
            CellFormat::Number(fmt) => Some(fmt.min_width),
            CellFormat::String(fmt) => Some(fmt.min_width),
            CellFormat::Binary(fmt) => Some(fmt.min_width),
            CellFormat::Bool(fmt) => Some(fmt.min_width),
        }
    }

    /// get max width
    pub fn get_max_width(&self) -> Option<usize> {
        match self {
            CellFormat::Number(fmt) => Some(fmt.max_width),
            CellFormat::String(fmt) => Some(fmt.max_width),
            CellFormat::Binary(fmt) => Some(fmt.max_width),
            CellFormat::Bool(fmt) => Some(fmt.max_width),
        }
    }
}

impl TryInto<NumberFormat> for CellFormat {
    type Error = FormatError;

    fn try_into(self) -> Result<NumberFormat, FormatError> {
        match self {
            CellFormat::Number(format) => Ok(format),
            _ => Err(FormatError::MismatchedFormatType("not a NumberFormat".to_string())),
        }
    }
}

impl TryInto<StringFormat> for CellFormat {
    type Error = FormatError;

    fn try_into(self) -> Result<StringFormat, FormatError> {
        match self {
            CellFormat::String(format) => Ok(format),
            _ => Err(FormatError::MismatchedFormatType("not a StringFormat".to_string())),
        }
    }
}

impl TryInto<BinaryFormat> for CellFormat {
    type Error = FormatError;

    fn try_into(self) -> Result<BinaryFormat, FormatError> {
        match self {
            CellFormat::Binary(format) => Ok(format),
            _ => Err(FormatError::MismatchedFormatType("not a BinaryFormat".to_string())),
        }
    }
}

impl TryInto<BoolFormat> for CellFormat {
    type Error = FormatError;

    fn try_into(self) -> Result<BoolFormat, FormatError> {
        match self {
            CellFormat::Bool(format) => Ok(format),
            _ => Err(FormatError::MismatchedFormatType("not a BoolFormat".to_string())),
        }
    }
}
