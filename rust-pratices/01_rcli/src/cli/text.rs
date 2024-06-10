use std::{fmt::Display, path, str::FromStr};

use clap::{Parser, Subcommand};

use crate::utils::validator::{validate_file, validate_path};

#[derive(Debug, Subcommand)]
pub enum TextSubcommand {
    Sign(TextSignOpts),
    Verify(TextVerifyOpts),
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-", value_parser = validate_file)]
    pub input: String,

    #[arg(short, long, value_parser = validate_file)]
    pub key: String,

    #[arg(short, long, value_parser = verify_sign_format, default_value = "Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value="-", value_parser = validate_file)]
    pub input: String,

    #[arg(short, long, value_parser = validate_file)]
    pub key: String,

    #[arg(short, long)]
    pub sig: String,

    #[arg(short, long, value_parser = verify_sign_format, default_value = "Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(short, long, value_parser = validate_path)]
    pub output: path::PathBuf,

    #[arg(short, long, value_parser = verify_sign_format, default_value = "Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => anyhow::bail!("not support {}", s),
        }
    }
}

impl Into<&str> for &TextSignFormat {
    fn into(self) -> &'static str {
        match self {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(self))
    }
}

fn verify_sign_format(format: &str) -> anyhow::Result<TextSignFormat> {
    TextSignFormat::from_str(format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_text_sign_format() {
        assert_eq!("blake3".to_string(), format!("{}", TextSignFormat::Blake3))
    }
}

