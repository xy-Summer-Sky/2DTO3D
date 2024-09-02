use crate::hex_utils::HexUtils;
use std::fs;
use std::path::Path;

pub struct Transformation {
    name: String,
    rules: Vec<String>, // Assuming rules are stored as strings for simplicity
}

impl Transformation {
    pub fn new(name: &str) -> Self {
        Transformation {
            name: name.to_string(),
            rules: Vec::new(),
        }
    }

    pub fn load_transformations<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("json")
            {
                let hex_path = HexUtils::path_to_hex(path.to_str().unwrap()); // Convert path to hex
                let decoded_path = HexUtils::hex_to_path(&hex_path).unwrap(); // Decode hex path back to string
                self.rules.push(decoded_path);
            }
        }

        Ok(())
    }
}
