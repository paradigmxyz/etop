use toolstr::{BinaryFormat, BoolFormat, NumberFormat, StringFormat};
use super::unknown_format::UnknownFormat;
use crate::EtopError;

#[derive(Debug, Clone)]
pub enum CellFormat {
    Number(NumberFormat),
    Binary(BinaryFormat),
    String(StringFormat),
    Bool(BoolFormat),
    Unknown(UnknownFormat),
}

impl CellFormat {
    pub fn min_width(self, min_width: usize) -> CellFormat {
        match self {
            CellFormat::Number(fmt) => CellFormat::Number(fmt.min_width(min_width)),
            CellFormat::String(fmt) => CellFormat::String(fmt.min_width(min_width)),
            CellFormat::Binary(fmt) => CellFormat::Binary(fmt.min_width(min_width)),
            CellFormat::Bool(fmt) => CellFormat::Bool(fmt.min_width(min_width)),
            CellFormat::Unknown(fmt) => CellFormat::Unknown(fmt.min_width(min_width)),
        }
    }

    pub fn max_width(self, max_width: usize) -> CellFormat {
        match self {
            CellFormat::Number(fmt) => CellFormat::Number(fmt.max_width(max_width)),
            CellFormat::String(fmt) => CellFormat::String(fmt.max_width(max_width)),
            CellFormat::Binary(fmt) => CellFormat::Binary(fmt.max_width(max_width)),
            CellFormat::Bool(fmt) => CellFormat::Bool(fmt.max_width(max_width)),
            CellFormat::Unknown(fmt) => CellFormat::Unknown(fmt.max_width(max_width)),
        }
    }

    pub fn get_min_width(&self) -> Option<usize> {
        match self {
            CellFormat::Number(fmt) => Some(fmt.min_width),
            CellFormat::String(fmt) => Some(fmt.min_width),
            CellFormat::Binary(fmt) => Some(fmt.min_width),
            CellFormat::Bool(fmt) => Some(fmt.min_width),
            CellFormat::Unknown(fmt) => fmt.min_width,
        }
    }

    pub fn get_max_width(&self) -> Option<usize> {
        match self {
            CellFormat::Number(fmt) => Some(fmt.max_width),
            CellFormat::String(fmt) => Some(fmt.max_width),
            CellFormat::Binary(fmt) => Some(fmt.max_width),
            CellFormat::Bool(fmt) => Some(fmt.max_width),
            CellFormat::Unknown(fmt) => fmt.max_width,
        }
    }
}

impl TryInto<NumberFormat> for CellFormat {
    type Error = EtopError;

    fn try_into(self) -> Result<NumberFormat, EtopError> {
        match self {
            CellFormat::Number(format) => Ok(format.clone()),
            CellFormat::Unknown(format) => Ok(format.into()),
            _ => Err(EtopError::MismatchedFormatType("not a NumberFormat".to_string())),
        }
    }
}

impl TryInto<StringFormat> for CellFormat {
    type Error = EtopError;

    fn try_into(self) -> Result<StringFormat, EtopError> {
        match self {
            CellFormat::String(format) => Ok(format.clone()),
            CellFormat::Unknown(format) => Ok(format.into()),
            _ => Err(EtopError::MismatchedFormatType("not a StringFormat".to_string())),
        }
    }
}

impl TryInto<BinaryFormat> for CellFormat {
    type Error = EtopError;

    fn try_into(self) -> Result<BinaryFormat, EtopError> {
        match self {
            CellFormat::Binary(format) => Ok(format.clone()),
            CellFormat::Unknown(format) => Ok(format.into()),
            _ => Err(EtopError::MismatchedFormatType("not a BinaryFormat".to_string())),
        }
    }
}

impl TryInto<BoolFormat> for CellFormat {
    type Error = EtopError;

    fn try_into(self) -> Result<BoolFormat, EtopError> {
        match self {
            CellFormat::Bool(format) => Ok(format.clone()),
            CellFormat::Unknown(format) => Ok(format.into()),
            _ => Err(EtopError::MismatchedFormatType("not a BoolFormat".to_string())),
        }
    }
}
