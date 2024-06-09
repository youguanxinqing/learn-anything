use clap::Parser;

mod process;
use process::process_base64::*;
use process::process_csv::*;
use process::process_genpass::*;
use process::process_text::process_text;

mod cli;
use cli::csv::CsvOpts;
use cli::genpass::GenpassOpts;

mod utils;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: cli::Command,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let result = match args.command {
        cli::Command::Csv(CsvOpts {
            file,
            delimiter,
            output,
            output_format,
        }) => process_ckv(&file, delimiter, &output, output_format),
        cli::Command::Genpass(GenpassOpts {
            length,
            symbol,
            number,
            lowercase,
            uppercase,
        }) => process_genpass(length, symbol, number, lowercase, uppercase),
        cli::Command::Base64(opts) => process_base64(opts),
        cli::Command::Text(subcommand) => process_text(subcommand),
        _ => Ok(()),
    };
    if result.is_err() {
        eprintln!("process err: {}", result.err().unwrap())
    }

    Ok(())
}
