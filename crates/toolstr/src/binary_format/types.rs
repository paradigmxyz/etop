#[cfg(test)]
#[path = "types_tests.rs"]
mod tests;

use crate::FormatError;

/// binary format specification
pub struct BinaryFormat {
    /// prefix of string
    pub prefix: bool,
    /// width of string, for padding
    pub width: usize,
    /// clip string if longer than width
    pub clip: bool,
    /// align binary to left or right
    pub align: BinaryAlign,
    /// use zeros for padding
    pub zero_padding: bool,
}

impl Default for BinaryFormat {
    fn default() -> BinaryFormat {
        BinaryFormat {
            prefix: true,
            width: 0,
            clip: false,
            align: BinaryAlign::Right,
            zero_padding: false,
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

        if total_length < self.width {
            let pad = if self.zero_padding {
                "0".repeat(self.width - total_length)
            } else {
                " ".repeat(self.width - total_length)
            };
            match (&self.align, self.zero_padding) {
                (BinaryAlign::Left, _) => Ok(format!("{}{}{}", prefix, s, pad)),
                (BinaryAlign::Right, true) => Ok(format!("{}{}{}", prefix, pad, s)),
                (BinaryAlign::Right, false) => Ok(format!("{}{}{}", pad, prefix, s)),
            }
        } else if self.clip {
            if self.width < 3 {
                return Err(FormatError::InvalidFormat(
                    "width too small for clipping".to_string(),
                ));
            };
            match s.get(0..(self.width - 3 - prefix.len())) {
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
