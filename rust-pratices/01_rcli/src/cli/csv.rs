use std::fmt;
use std::path;
use std::str::FromStr;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[clap(long, value_parser = validate_file_existed)]
    pub file: String,

    #[clap(long, default_value = ",")]
    pub delimiter: char,

    #[clap(long, default_value = "output")]
    pub output: String,

    #[clap(long, default_value = "json", value_parser = validate_output_format)]
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl From<OutputFormat> for &str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => anyhow::Ok(OutputFormat::Json),
            "yaml" => anyhow::Ok(OutputFormat::Yaml),
            other => anyhow::bail!("Not support {}", other),
        }
    }
}

impl From<&str> for OutputFormat {
    fn from(value: &str) -> Self {
        value.parse().unwrap()
    }
}

fn validate_file_existed(file: &str) -> anyhow::Result<String> {
    if !path::Path::new(file).exists() {
        anyhow::bail!(format!("Not found {}", file))
    } else {
        anyhow::Ok(file.into())
    }
}

fn validate_output_format(format: &str) -> anyhow::Result<OutputFormat> {
    format.parse()
}
