use std::path;

use anyhow::Result;

pub fn validate_file(input: &str) -> Result<String, String> {
    if input == "-" || path::Path::new(input).exists() {
        Ok(input.to_owned())
    } else {
        Err(format!("not found file: {}", input))
    }
}

pub fn validate_path(path: &str) -> anyhow::Result<path::PathBuf> {
    let p = path::Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(p.to_path_buf())
    } else {
        anyhow::bail!("Not found dir: {}", path)
    }
}
