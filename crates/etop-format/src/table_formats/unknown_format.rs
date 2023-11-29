use crate::{BinaryFormat, BoolFormat, NumberFormat, StringFormat};

/// unknown format
#[derive(Debug, Clone)]
pub struct UnknownFormat {
    /// min width
    pub min_width: Option<usize>,
    /// max width
    pub max_width: Option<usize>,
}

impl UnknownFormat {
    /// min width
    pub fn min_width(mut self, width: usize) -> UnknownFormat {
        self.min_width = Some(width);
        self
    }

    /// max width
    pub fn max_width(mut self, width: usize) -> UnknownFormat {
        self.max_width = Some(width);
        self
    }
}

impl From<UnknownFormat> for NumberFormat {
    fn from(unknown_format: UnknownFormat) -> Self {
        NumberFormat::new()
            .min_width_option(unknown_format.min_width)
            .max_width_option(unknown_format.max_width)
    }
}

impl From<UnknownFormat> for BinaryFormat {
    fn from(unknown_format: UnknownFormat) -> Self {
        BinaryFormat::new()
            .min_width_option(unknown_format.min_width)
            .max_width_option(unknown_format.max_width)
    }
}

impl From<UnknownFormat> for StringFormat {
    fn from(unknown_format: UnknownFormat) -> Self {
        StringFormat::new()
            .min_width_option(unknown_format.min_width)
            .max_width_option(unknown_format.max_width)
    }
}

impl From<UnknownFormat> for BoolFormat {
    fn from(unknown_format: UnknownFormat) -> Self {
        BoolFormat::new()
            .min_width_option(unknown_format.min_width)
            .max_width_option(unknown_format.max_width)
    }
}
