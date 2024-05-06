use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[clap(long, default_value_t = 8, value_parser = validate_length_more_than_zero)]
    pub length: u8,

    #[clap(long, default_value_t = false)]
    pub symbol: bool,

    #[clap(long, default_value_t = true)]
    pub number: bool,

    #[clap(long, default_value_t = true)]
    pub lowercase: bool,

    #[clap(long, default_value_t = false)]
    pub uppercase: bool,
}

fn validate_length_more_than_zero(length: &str) -> anyhow::Result<u8> {
    match length.parse() {
        Err(e) => Err(anyhow::Error::from(e)),
        Ok(v) => Ok(v),
    }
}
