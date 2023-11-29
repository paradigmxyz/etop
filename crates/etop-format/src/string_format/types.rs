#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

use crate::FormatError;

/// string format specification
#[derive(Debug, Clone)]
pub struct StringFormat {
    /// min_width of string, for padding
    pub min_width: usize,
    /// max_width of string, for padding
    pub max_width: usize,
    /// align string to left or right
    pub align: StringAlign,
    /// fill padding char
    pub fill_char: char,
}

impl Default for StringFormat {
    fn default() -> StringFormat {
        StringFormat {
            min_width: 0,
            max_width: usize::MAX,
            align: StringAlign::Right,
            fill_char: ' ',
        }
    }
}

/// alignment of string data
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StringAlign {
    /// left align
    Left,
    /// right align
    Right,
}

impl StringFormat {
    /// format string data
    pub fn format_option<T: AsRef<str>, S: AsRef<str>>(
        &self,
        s: Option<T>,
        none_str: S,
    ) -> Result<String, FormatError> {
        match s {
            Some(data) => self.format(data),
            None => Ok(none_str.as_ref().to_string()),
        }
    }

    /// format string data
    pub fn format<T: AsRef<str>>(&self, s: T) -> Result<String, FormatError> {
        let s = s.as_ref();
        if s.len() < self.min_width {
            let pad = self.fill_char.to_string().repeat(self.min_width - s.len());
            match &self.align {
                StringAlign::Left => Ok(format!("{}{}", s, pad)),
                StringAlign::Right => Ok(format!("{}{}", pad, s)),
            }
        } else if s.len() > self.max_width {
            if self.max_width < 3 {
                return Err(FormatError::InvalidFormat(
                    "min_width too small for clipping".to_string(),
                ));
            };
            match s.get(0..(self.max_width - 3)) {
                Some(s) => Ok(format!("{}...", s)),
                None => Err(FormatError::InvalidFormat(
                    "could not take slice of string".to_string(),
                )),
            }
        } else {
            Ok(s.to_string())
        }
    }
}
