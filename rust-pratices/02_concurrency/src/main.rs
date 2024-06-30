use std::io;

use async_std::{fs::read_to_string, task::block_on};

#[async_std::main]
async fn main() -> Result<(), io::Error> {
    let f = async {
        let s = read_to_string("Cargo.toml").await?;  // 测试中文
        println!("{}", s);
        Ok::<(), io::Error>(())
    };

    block_on(f)?;

    Ok(())
}
