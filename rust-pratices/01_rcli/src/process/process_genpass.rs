use anyhow;
use zxcvbn::zxcvbn;

use crate::utils::password;

pub fn process_genpass(
    length: u8,
    symbol: bool,
    number: bool,
    lowercase: bool,
    uppercase: bool,
) -> anyhow::Result<()> {
    let password = password::process_genpass(length, symbol, number, lowercase, uppercase)?;
    let score = zxcvbn(&password, &[]).unwrap().score();
    println!("gen password is: {}, level is {}", password, score);

    Ok(())
}
