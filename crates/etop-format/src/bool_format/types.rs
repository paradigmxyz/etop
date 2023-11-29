#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

use crate::FormatError;

/// bool format specification
#[derive(Debug, Clone)]
pub struct BoolFormat {
    /// min_width of bool, for padding
    pub min_width: usize,
    /// max_width of bool, for padding
    pub max_width: usize,
    /// align bool to left or right
    pub align: BoolAlign,
    /// fill padding char
    pub fill_char: char,
    /// true string
    pub true_text: String,
    /// false string
    pub false_text: String,
}

impl Default for BoolFormat {
    fn default() -> BoolFormat {
        BoolFormat {
            min_width: 0,
            max_width: usize::MAX,
            align: BoolAlign::Right,
            fill_char: ' ',
            true_text: "true".to_string(),
            false_text: "false".to_string(),
        }
    }
}

/// alignment of bool data
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BoolAlign {
    /// left align
    Left,
    /// right align
    Right,
}

impl BoolFormat {
    /// format bool data
    pub fn format_option<S: AsRef<str>>(
        &self,
        s: Option<bool>,
        none_str: S,
    ) -> Result<String, FormatError> {
        match s {
            Some(data) => self.format(data),
            None => Ok(none_str.as_ref().to_string()),
        }
    }

    /// format bool data
    pub fn format(&self, s: bool) -> Result<String, FormatError> {
        let s = match s {
            true => self.true_text.clone(),
            false => self.false_text.clone(),
        };
        if s.len() < self.min_width {
            let pad = self.fill_char.to_string().repeat(self.min_width - s.len());
            match &self.align {
                BoolAlign::Left => Ok(format!("{}{}", s, pad)),
                BoolAlign::Right => Ok(format!("{}{}", pad, s)),
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
                    "could not take slice of bool".to_string(),
                )),
            }
        } else {
            Ok(s.to_string())
        }
    }
}
