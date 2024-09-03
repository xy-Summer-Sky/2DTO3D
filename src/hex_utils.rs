// src/hex_utils.rs
pub struct HexUtils;

impl HexUtils {
    pub fn path_to_hex(path: &str) -> String {
        path.as_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }

    pub fn hex_to_path(hex: &str) -> Result<String, HexToPathError> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(HexToPathError::from))
            .collect::<Result<Vec<u8>, _>>()
            .and_then(|bytes| String::from_utf8(bytes).map_err(HexToPathError::from))
    }
}

#[derive(Debug)]
pub enum HexToPathError {
    Utf8Error(std::string::FromUtf8Error),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::string::FromUtf8Error> for HexToPathError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        HexToPathError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for HexToPathError {
    fn from(err: std::num::ParseIntError) -> Self {
        HexToPathError::ParseIntError(err)
    }
}
