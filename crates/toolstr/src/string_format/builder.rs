use super::types::{StringAlign, StringFormat};

impl StringFormat {
    /// create new number format
    pub fn new() -> StringFormat {
        StringFormat::default()
    }

    // width

    /// set width
    pub fn width(mut self, width: usize) -> StringFormat {
        self.min_width = width;
        self.max_width = width;
        self
    }

    /// set min_width
    pub fn min_width(mut self, min_width: usize) -> StringFormat {
        self.min_width = min_width;
        self
    }

    /// set max_width
    pub fn max_width(mut self, max_width: usize) -> StringFormat {
        self.max_width = max_width;
        self
    }

    // align

    /// left align
    pub fn left_align(mut self) -> StringFormat {
        self.align = StringAlign::Left;
        self
    }

    /// right align
    pub fn right_align(mut self) -> StringFormat {
        self.align = StringAlign::Right;
        self
    }

    // fill char

    /// add fill char
    pub fn fill_char(mut self, fill_char: char) -> StringFormat {
        self.fill_char = fill_char;
        self
    }
}
