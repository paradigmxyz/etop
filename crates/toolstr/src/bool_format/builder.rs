use super::types::{BoolAlign, BoolFormat};

impl BoolFormat {
    /// create new number format
    pub fn new() -> BoolFormat {
        BoolFormat::default()
    }

    // width

    /// set width
    pub fn width(mut self, width: usize) -> BoolFormat {
        self.min_width = width;
        self.max_width = width;
        self
    }

    /// set min_width
    pub fn min_width(mut self, min_width: usize) -> BoolFormat {
        self.min_width = min_width;
        self
    }

    /// set max_width
    pub fn max_width(mut self, max_width: usize) -> BoolFormat {
        self.max_width = max_width;
        self
    }

    /// set width
    pub fn width_option(self, width: Option<usize>) -> BoolFormat {
        self.min_width_option(width).max_width_option(width)
    }

    /// set min_width
    pub fn min_width_option(mut self, width: Option<usize>) -> BoolFormat {
        match width {
            Some(width) => {
                self.min_width = width;
                self
            }
            None => self,
        }
    }

    /// set max_width
    pub fn max_width_option(mut self, width: Option<usize>) -> BoolFormat {
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
    pub fn left_align(mut self) -> BoolFormat {
        self.align = BoolAlign::Left;
        self
    }

    /// right align
    pub fn right_align(mut self) -> BoolFormat {
        self.align = BoolAlign::Right;
        self
    }

    // fill char

    /// add fill char
    pub fn fill_char(mut self, fill_char: char) -> BoolFormat {
        self.fill_char = fill_char;
        self
    }

    // value formats

    /// true format
    pub fn true_text(mut self, true_text: String) -> BoolFormat {
        self.true_text = true_text;
        self
    }

    /// false format
    pub fn false_text(mut self, false_text: String) -> BoolFormat {
        self.false_text = false_text;
        self
    }
}
