use std::{fmt, path};

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Base64Subcommand {
    Encode(Base64Opts),
    Decode(Base64Opts),
}

#[derive(Debug, Parser)]
pub struct Base64Opts {
    #[clap(short, long, default_value = "-", value_parser = validate_input)]
    pub input: String,

    #[clap(long, default_value = "normal", value_parser = validate_formatter)]
    pub format: Base64Formatter,
}

fn validate_input(input: &str) -> Result<String, String> {
    if input == "-" || path::Path::new(input).exists() {
        Ok(input.to_owned())
    } else {
        Err(format!("not found file: {}", input))
    }
}

fn validate_formatter(formatter: &str) -> Result<Base64Formatter, String> {
    match formatter.to_lowercase().as_str() {
        "normal" => Ok(Base64Formatter::NORMAL),
        "urlsafe" => Ok(Base64Formatter::URLSAFE),
        other => {
            return Err(format!("not support formatter: {}", other));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Formatter {
    NORMAL,
    URLSAFE,
}

impl fmt::Display for Base64Formatter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            Self::NORMAL => "normal",
            Self::URLSAFE => "urlsafe",
        };
        write!(f, "{}", v)
    }
}
