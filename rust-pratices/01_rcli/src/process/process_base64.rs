use std::io::Read;

use crate::cli::base64::*;
use crate::utils::reader::get_reader;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};

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

fn read_all(reader: &mut Box<dyn Read>) -> anyhow::Result<String> {
    let mut input_content = Vec::new();
    reader.as_mut().read_to_end(&mut input_content)?;
    let input_content = String::from_utf8(input_content)?;
    Ok(input_content.trim().to_owned())
}

fn process_base64_encode(input: &str, formatter: Base64Formatter) -> anyhow::Result<()> {
    println!("encode -- input: {}, formatter: {}", input, formatter);

    let reader = get_reader(input)?;
    let input_content = read_all(&mut Box::new(reader))?;

    let encoder = match formatter {
        Base64Formatter::NORMAL => STANDARD,
        Base64Formatter::URLSAFE => URL_SAFE,
    };
    let encode_string = encoder.encode(input_content);
    println!("{}", encode_string);

    Ok(())
}

fn process_base64_decode(input: &str, formatter: Base64Formatter) -> anyhow::Result<()> {
    println!("decode -- input: {}, formatter: {}", input, formatter);

    let reader = get_reader(input)?;
    let input_content = read_all(&mut Box::new(reader))?;

    let encoder = match formatter {
        Base64Formatter::NORMAL => STANDARD,
        Base64Formatter::URLSAFE => URL_SAFE,
    };
    let decode_string = encoder.decode(input_content)?;
    println!("{}", String::from_utf8(decode_string)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_normal_encode() {
        assert!(process_base64_encode("assets/normal.txt", Base64Formatter::NORMAL).is_ok());
    }

    #[test]
    fn test_base64_urlsafe_encode() {
        assert!(process_base64_encode("assets/normal.txt", Base64Formatter::URLSAFE).is_ok());
    }

    #[test]
    fn test_base64_normal_decode() {}

    #[test]
    fn test_base64_urlsafe_decode() {}
}
