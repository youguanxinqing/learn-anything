pub mod base64;
pub mod csv;
pub mod genpass;

use clap::{command, Subcommand};

use self::base64::Base64Subcommand;
use self::csv::CsvOpts;
use self::genpass::GenpassOpts;

#[derive(Debug, Subcommand)]
pub enum Command {
    Csv(CsvOpts),

    Genpass(GenpassOpts),

    #[command(subcommand)]
    Base64(Base64Subcommand),
}
