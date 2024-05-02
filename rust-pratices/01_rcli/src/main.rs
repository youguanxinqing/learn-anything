use core::fmt;
use std::{fs, io::Write, path, str::FromStr};

use clap::{Parser, Subcommand};
use csv::ReaderBuilder;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Csv {
        #[clap(long, value_parser = validate_file_existed)]
        file: String,

        #[clap(long, default_value = ",")]
        delimiter: char,

        #[clap(long, default_value = "output")]
        output: String,

        #[clap(long, default_value = "json", value_parser = validate_output_format)]
        output_format: OutputFormat,
    },
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Csv {
            file,
            delimiter,
            output,
            output_format,
        } => {
            let f_handler = fs::File::open(file)?;
            let mut rdr = ReaderBuilder::new()
                .delimiter(delimiter as u8)
                .from_reader(f_handler);
            let header = rdr.headers()?.clone();

            let mut out = fs::File::create(format!("{}.{}", output, output_format.clone()))?;
            for line in rdr.records() {
                let line = line?;
                let line = header
                    .iter()
                    .zip(line.iter())
                    .collect::<serde_json::Value>();
                let line = match output_format {
                    OutputFormat::Json => serde_json::to_string(&line)?,
                    OutputFormat::Yaml => serde_yaml::to_string(&line)?,
                };
                out.write(line.as_bytes())?;
                out.write(b"\n")?;
            }
        }
    };

    Ok(())
}
