#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

use crate::FormatError;

/// binary format specification
#[derive(Debug, Clone)]
pub struct BinaryFormat {
    /// prefix of string
    pub prefix: bool,
    /// min_width of string, for padding
    pub min_width: usize,
    /// max_width of string, for padding
    pub max_width: usize,
    /// align binary to left or right
    pub align: BinaryAlign,
    /// fill padding char
    pub fill_char: char,
}

impl Default for BinaryFormat {
    fn default() -> BinaryFormat {
        BinaryFormat {
            prefix: true,
            min_width: 0,
            max_width: usize::MAX,
            align: BinaryAlign::Right,
            fill_char: ' ',
        }
    }
}

/// alignment of binary data
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BinaryAlign {
    /// left align
    Left,
    /// right align
    Right,
}

impl BinaryFormat {
    /// format binary data
    pub fn format<T: AsRef<[u8]>>(&self, data: T) -> Result<String, FormatError> {
        let s = bytes_to_hex(data);

        let (total_length, prefix) = if self.prefix {
            (s.len() + 2, "0x")
        } else {
            (s.len(), "")
        };

        if total_length < self.min_width {
            let pad = self
                .fill_char
                .to_string()
                .repeat(self.min_width - total_length);
            let zero_padding = self.fill_char == '0';
            match (&self.align, zero_padding) {
                (BinaryAlign::Left, _) => Ok(format!("{}{}{}", prefix, s, pad)),
                (BinaryAlign::Right, true) => Ok(format!("{}{}{}", prefix, pad, s)),
                (BinaryAlign::Right, false) => Ok(format!("{}{}{}", pad, prefix, s)),
            }
        } else if total_length > self.max_width {
            if self.max_width < 3 {
                return Err(FormatError::InvalidFormat(
                    "min_width too small for clipping".to_string(),
                ));
            };
            match s.get(0..(self.max_width - 3 - prefix.len())) {
                Some(s) => Ok(format!("{}{}...", prefix, s)),
                None => Err(FormatError::InvalidFormat(
                    "could not take slice of string".to_string(),
                )),
            }
        } else {
            Ok(format!("{}{}", prefix, s))
        }
    }
}

/// convert bytes to raw hex string
fn bytes_to_hex<T: AsRef<[u8]>>(data: T) -> String {
    let hex_chars = "0123456789abcdef".as_bytes();
    let bytes = data.as_ref();

    let mut hex_string = String::with_capacity(bytes.len() * 2);

    for &byte in bytes {
        hex_string.push(hex_chars[(byte >> 4) as usize] as char);
        hex_string.push(hex_chars[(byte & 0xf) as usize] as char);
    }

    hex_string
}
