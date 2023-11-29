use super::types::{FormatType, NumberAlign, NumberFormat, Sign, Timezone};

impl NumberFormat {
    /// create new number format
    pub fn new() -> NumberFormat {
        NumberFormat::default()
    }

    // zero padding

    /// add zero padding
    pub fn zero_padding(mut self) -> NumberFormat {
        self.zero_padding = true;
        self
    }

    /// remove zero padding
    pub fn no_zero_padding(mut self) -> NumberFormat {
        self.zero_padding = false;
        self
    }

    // fill

    /// set fill char
    pub fn fill(mut self, fill_char: char) -> NumberFormat {
        self.fill = fill_char;
        self
    }

    // align

    /// left align
    pub fn left_align(mut self) -> NumberFormat {
        self.align = NumberAlign::Left;
        self
    }

    /// right align
    pub fn right_align(mut self) -> NumberFormat {
        self.align = NumberAlign::Right;
        self
    }

    /// center align
    pub fn center_align(mut self) -> NumberFormat {
        self.align = NumberAlign::Center;
        self
    }

    /// signed right align
    pub fn left_sign_right_align(mut self) -> NumberFormat {
        self.align = NumberAlign::SignedRight;
        self
    }

    // sign

    /// always add sign
    pub fn unsigned(mut self) -> NumberFormat {
        self.sign = Sign::OnlyNegative;
        self
    }

    /// always add sign
    pub fn signed(mut self) -> NumberFormat {
        self.sign = Sign::Always;
        self
    }

    /// unsigned but with space
    pub fn unsigned_space(mut self) -> NumberFormat {
        self.sign = Sign::SpaceOrDash;
        self
    }

    // type_prefix

    /// add type prefix
    pub fn type_prefix(mut self) -> NumberFormat {
        self.type_prefix = true;
        self
    }

    /// no type prefix
    pub fn no_type_prefix(mut self) -> NumberFormat {
        self.type_prefix = false;
        self
    }

    // width

    /// set width
    pub fn width(mut self, width: usize) -> NumberFormat {
        self.min_width = width;
        self.max_width = width;
        self
    }

    /// set min_width
    pub fn min_width(mut self, min_width: usize) -> NumberFormat {
        self.min_width = min_width;
        self
    }

    /// set max_width
    pub fn max_width(mut self, max_width: usize) -> NumberFormat {
        self.max_width = max_width;
        self
    }

    /// set width
    pub fn width_option(self, width: Option<usize>) -> NumberFormat {
        self.min_width_option(width).max_width_option(width)
    }

    /// set min_width
    pub fn min_width_option(mut self, width: Option<usize>) -> NumberFormat {
        match width {
            Some(width) => {
                self.min_width = width;
                self
            }
            None => self,
        }
    }

    /// set max_width
    pub fn max_width_option(mut self, width: Option<usize>) -> NumberFormat {
        match width {
            Some(width) => {
                self.max_width = width;
                self
            }
            None => self,
        }
    }

    // commas

    /// show commas
    pub fn commas(mut self) -> NumberFormat {
        self.commas = true;
        self
    }

    /// do not show commas
    pub fn no_commas(mut self) -> NumberFormat {
        self.commas = false;
        self
    }

    // precision

    /// set precision
    pub fn precision(mut self, precision: usize) -> NumberFormat {
        self.precision = precision;
        self
    }

    // timezone

    /// use local timezone
    pub fn timezone_local(mut self) -> NumberFormat {
        self.timezone = Timezone::Local;
        self
    }

    /// use utc timezone
    pub fn timezone_utc(mut self) -> NumberFormat {
        self.timezone = Timezone::Utc;
        self
    }

    // format_type

    /// format as scientific notation
    pub fn scientific_notation(mut self) -> NumberFormat {
        self.format_type = FormatType::Exponent;
        self
    }

    /// format as SI (order of magnitude)
    pub fn si(mut self) -> NumberFormat {
        self.format_type = FormatType::SI;
        self
    }

    /// format as pecentage
    pub fn percentage(mut self) -> NumberFormat {
        self.format_type = FormatType::Percentage;
        self
    }

    /// format as binary
    pub fn binary(mut self) -> NumberFormat {
        self.format_type = FormatType::Binary;
        self
    }

    /// format as octal
    pub fn octal(mut self) -> NumberFormat {
        self.format_type = FormatType::Octal;
        self
    }

    /// format as hex
    pub fn hex(mut self) -> NumberFormat {
        self.format_type = FormatType::Octal;
        self
    }

    /// format as integer order of magnitude
    pub fn integer_oom(mut self) -> NumberFormat {
        self.format_type = FormatType::IntegerOrderOfMagnitude;
        self
    }

    /// format as float order of magnitude
    pub fn float_oom(mut self) -> NumberFormat {
        self.format_type = FormatType::FloatOrderOfMagnitude;
        self
    }

    /// format as float order of magnitude
    pub fn timestamp(mut self) -> NumberFormat {
        self.format_type = FormatType::TimestampPretty;
        self
    }

    /// set format type
    pub fn format_type(mut self, format_type: &FormatType) -> NumberFormat {
        self.format_type = format_type.clone();
        self
    }
}
