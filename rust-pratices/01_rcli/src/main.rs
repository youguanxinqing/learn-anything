use std::{fmt::Display, fs, path};

use clap::{Parser, Subcommand};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Author")]
    author: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Price")]
    price: f32,
}

impl Book {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, author: {}, date: {}, price: {}",
            self.name, self.author, self.date, self.price
        )
    }
}

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

        #[clap(long)]
        to_json: bool,
    },
}

fn validate_file_existed(file: &str) -> Result<String, String> {
    if !path::Path::new(file).exists() {
        Err(format!("Not found {}", file))
    } else {
        Ok(file.into())
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Csv {
            file,
            delimiter,
            to_json,
        } => {
            let f_handler = fs::File::open(file)?;
            let mut rdr = ReaderBuilder::new()
                .delimiter(delimiter as u8)
                .from_reader(f_handler);
            for line in rdr.deserialize() {
                let line: Book = line?;

                if to_json {
                    println!("{}", line.to_json());
                } else {
                    println!("{}", line);
                }
            }
        }
    };

    Ok(())
}
