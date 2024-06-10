use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::utils::validator::{validate_path};


#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    Serve(HttpServeOpts)
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = validate_path, default_value = ".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
