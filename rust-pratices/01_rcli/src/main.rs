use std::{path, u8};

use clap::{Parser, Subcommand};

mod process;
use process::process_csv::*;
use process::process_genpass::*;

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
    Genpass {
        #[clap(long, default_value_t = 8, value_parser = validate_length_more_than_zero)]
        length: u8,

        #[clap(long, default_value_t = false)]
        symbol: bool,

        #[clap(long, default_value_t = true)]
        number: bool,

        #[clap(long, default_value_t = true)]
        lowercase: bool,

        #[clap(long, default_value_t = false)]
        uppercase: bool,
    },
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

fn validate_length_more_than_zero(length: &str) -> anyhow::Result<u8> {
    match length.parse() {
        Err(e) => Err(anyhow::Error::from(e)),
        Ok(v) => Ok(v),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let result = match args.command {
        Command::Csv {
            file,
            delimiter,
            output,
            output_format,
        } => process_ckv(&file, delimiter, &output, output_format),
        Command::Genpass {
            length,
            symbol,
            number,
            lowercase,
            uppercase,
        } => process_genpass(length, symbol, number, lowercase, uppercase),
        _ => Ok(()),
    };
    if result.is_err() {
        eprintln!("process err: {}", result.err().unwrap())
    }

    Ok(())
}
