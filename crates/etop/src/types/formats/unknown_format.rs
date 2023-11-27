use toolstr::{BinaryFormat, BoolFormat, NumberFormat, StringFormat};

#[derive(Debug, Clone)]
pub struct UnknownFormat {
    pub min_width: Option<usize>,
    pub max_width: Option<usize>,
}

impl UnknownFormat {
    pub fn min_width(mut self, width: usize) -> UnknownFormat {
        self.min_width = Some(width);
        self
    }

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
