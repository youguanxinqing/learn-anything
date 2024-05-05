use anyhow;
use rand::prelude::*;
use rand::thread_rng;
use zxcvbn::zxcvbn;

const NUMBER: &[u8] = b"1234567890";
const ALPHA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const SYMBOL: &[u8] = b",.;?&^%$#@*_-";

pub fn process_genpass(
    length: u8,
    symbol: bool,
    number: bool,
    lowercase: bool,
    uppercase: bool,
) -> anyhow::Result<()> {
    let mut rng = thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).unwrap());
    }

    if lowercase {
        chars.extend_from_slice(ALPHA);
        password.push(*ALPHA.choose(&mut rng).unwrap());
    }

    if uppercase {
        let uppercase_table = ALPHA
            .iter()
            .map(|s| s.to_ascii_uppercase())
            .collect::<Vec<u8>>();
        chars.extend_from_slice(&uppercase_table);
        password.push(*uppercase_table.choose(&mut rng).unwrap());
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    for _ in 0..(length as usize - password.len()) {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }
    password.shuffle(&mut rng);

    let password = String::from_utf8(password).unwrap();
    let score = zxcvbn(&password, &[]).unwrap().score();
    println!("gen password is: {}, level is {}", password, score);

    Ok(())
}
