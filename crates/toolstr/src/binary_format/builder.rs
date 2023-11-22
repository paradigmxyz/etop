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
        self.width = width;
        self
    }

    // clip

    /// clip
    pub fn clip(mut self) -> BinaryFormat {
        self.clip = true;
        self
    }

    /// no clip
    pub fn no_clip(mut self) -> BinaryFormat {
        self.clip = false;
        self
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

    // zero padding

    /// add zero padding
    pub fn zero_padding(mut self) -> BinaryFormat {
        self.zero_padding = true;
        self
    }

    /// remove zero padding
    pub fn no_zero_padding(mut self) -> BinaryFormat {
        self.zero_padding = false;
        self
    }
}
