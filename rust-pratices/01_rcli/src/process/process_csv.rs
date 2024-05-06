use std::fs;
use std::io::Write;

use anyhow::{self, Ok};
use csv::ReaderBuilder;

use crate::cli::csv::OutputFormat;

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
