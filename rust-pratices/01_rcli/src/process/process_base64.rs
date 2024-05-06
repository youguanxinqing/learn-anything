use std::{io::Read};

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

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
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
    
    println!("input_content: {:?}", input_content);

    Ok(())
}

fn process_base64_decode(input: &str, formatter: Base64Formatter) -> anyhow::Result<()> {
    println!("decode -- input: {}, formatter: {}", input, formatter);

    let reader = get_reader(input)?;
    let input_content = read_all(&mut Box::new(reader))?;

    println!("input_content: {:?}", input_content);
    
    Ok(())
}
