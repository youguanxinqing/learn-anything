pub mod base64;
pub mod csv;
pub mod genpass;
pub mod http;
pub mod text;

use clap::{command, Subcommand};

use self::base64::Base64Subcommand;
use self::csv::CsvOpts;
use self::genpass::GenpassOpts;
use self::http::HttpSubCommand;
use self::text::TextSubcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    Csv(CsvOpts),

    Genpass(GenpassOpts),

    #[command(subcommand)]
    Base64(Base64Subcommand),

    #[command(subcommand)]
    Text(TextSubcommand),

    #[command(subcommand)]
    Http(HttpSubCommand),
}
