use crate::cli::base64::*;

pub fn process_base64(cmd: Base64Subcommand) -> anyhow::Result<()> {
    match cmd {
        Base64Subcommand::Encode(Base64Opts { input, format }) => {
            process_base64_encode(&input, format)
        }
        Base64Subcommand::Decode(Base64Opts { input, format }) => {
            process_base64_decode(&input, format)
        }
    }
}

fn process_base64_encode(input: &str, formatter: Base64Formatter) -> anyhow::Result<()> {
    println!("encode -- input: {}, formatter: {}", input, formatter);
    Ok(())
}

fn process_base64_decode(input: &str, formatter: Base64Formatter) -> anyhow::Result<()> {
    println!("decode -- input: {}, formatter: {}", input, formatter);
    Ok(())
}
