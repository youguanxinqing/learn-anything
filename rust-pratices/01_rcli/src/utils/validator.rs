use std::path;

use anyhow::Result;

pub fn validate_file(input: &str) -> Result<String, String> {
    if input == "-" || path::Path::new(input).exists() {
        Ok(input.to_owned())
    } else {
        Err(format!("not found file: {}", input))
    }
}
