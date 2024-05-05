use std::path;

use clap::{Parser, Subcommand};

mod process;
use process::process_csv::{self, *};

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
    
    let result = match args.command {
        Command::Csv {
            file,
            delimiter,
            output,
            output_format,
        } => {
            process_csv::process_ckv(&file, delimiter, &output, output_format)
        }
    };
    if result.is_err() {
        eprintln!("process err: {}", result.err().unwrap())
    }

    Ok(())
}
