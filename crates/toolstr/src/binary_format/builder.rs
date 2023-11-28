use super::types::{BinaryAlign, BinaryFormat};

impl BinaryFormat {
    /// create new number format
    pub fn new() -> BinaryFormat {
        BinaryFormat::default()
    }

    // prefix

    /// prefix
    pub fn prefix(mut self) -> BinaryFormat {
        self.prefix = true;
        self
    }

    /// no prefix
    pub fn no_prefix(mut self) -> BinaryFormat {
        self.prefix = false;
        self
    }

    // width

    /// set width
    pub fn width(mut self, width: usize) -> BinaryFormat {
        self.min_width = width;
        self.max_width = width;
        self
    }

    /// set min_width
    pub fn min_width(mut self, min_width: usize) -> BinaryFormat {
        self.min_width = min_width;
        self
    }

    /// set max_width
    pub fn max_width(mut self, max_width: usize) -> BinaryFormat {
        self.max_width = max_width;
        self
    }

    /// set width
    pub fn width_option(self, width: Option<usize>) -> BinaryFormat {
        self.min_width_option(width).max_width_option(width)
    }

    /// set min_width
    pub fn min_width_option(mut self, width: Option<usize>) -> BinaryFormat {
        match width {
            Some(width) => {
                self.min_width = width;
                self
            }
            None => self,
        }
    }

    /// set max_width
    pub fn max_width_option(mut self, width: Option<usize>) -> BinaryFormat {
        match width {
            Some(width) => {
                self.max_width = width;
                self
            }
            None => self,
        }
    }

    // align

    /// left align
    pub fn left_align(mut self) -> BinaryFormat {
        self.align = BinaryAlign::Left;
        self
    }

    /// right align
    pub fn right_align(mut self) -> BinaryFormat {
        self.align = BinaryAlign::Right;
        self
    }

    // fill char

    /// add fill char
    pub fn fill_char(mut self, fill_char: char) -> BinaryFormat {
        self.fill_char = fill_char;
        self
    }
}
