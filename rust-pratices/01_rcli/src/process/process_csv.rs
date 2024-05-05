use std::fs;
use std::fmt;
use std::str::FromStr;
use std::io::Write;

use anyhow::{self, Ok};
use csv::ReaderBuilder;

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

pub fn process_ckv(
    file: &str,
    delimiter: char,
    output: &str,
    output_format: OutputFormat,
) -> anyhow::Result<()> {

    let f_handler = fs::File::open(file)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_reader(f_handler);
    let header = rdr.headers()?.clone();

    let mut ret = Vec::with_capacity(10);
    for line in rdr.records() {
        let line = line?;
        let line = header
            .iter()
            .zip(line.iter())
            .collect::<serde_json::Value>();
        ret.push(line);
    }

    let lines = match output_format.into() {
        OutputFormat::Json => serde_json::to_string(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    let mut out = fs::File::create(format!("{}.{}", output, output_format))?;
    out.write(lines.as_bytes())?;


    Ok(())
}
