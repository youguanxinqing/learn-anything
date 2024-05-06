pub mod csv;
pub mod genpass;

use clap::Subcommand;

use self::csv::CsvOpts;
use self::genpass::GenpassOpts;

#[derive(Debug, Subcommand)]
pub enum Command {
    Csv(CsvOpts),
    Genpass(GenpassOpts),
}
