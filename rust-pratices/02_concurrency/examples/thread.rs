use std::{
    fmt::Display,
    sync::mpsc::{channel, Sender},
    thread::{self, sleep},
    time::Duration,
};

use anyhow::{anyhow, Ok};

const THREAD_NUM: i32 = 4;

#[derive(Debug)]
struct Msg {
    idx: i32,
    value: i32,
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "idx={}, value={}", self.idx, self.value)
    }
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = channel::<Msg>();

    for idx in 0..THREAD_NUM {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }
    drop(tx);

    for msg in rx {
        // for in will end when all tx are droped.
        println!("recv: {:?}", msg);
    }
    println!("recv exit");

    Ok(())
}

fn producer(idx: i32, tx: Sender<Msg>) -> anyhow::Result<()> {
    loop {
        let num = rand::random::<u8>() as i32;
        tx.send(Msg { idx, value: num })
            .map_err(|e| anyhow!("send msg err: {}", e))?;

        if num % 10 == 0 {
            break;
        }

        sleep(Duration::from_millis(num as u64 * 10));
    }

    println!("producer exit: idx={}", idx);
    Ok(())
}
