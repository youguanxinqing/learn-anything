use std::{sync::mpsc::channel, thread};

fn main() {
    let (tx, rx) = channel();
    let sender = thread::spawn(move || {
        tx.send("Hello, A!").expect("Failed to send msg!");
    });
    let reciver = thread::spawn(move || {
        let msg = rx.recv().expect("Failed to recv msg!");
        println!("B: {}", msg);
    });

    sender.join().expect("sender panic");
    reciver.join().expect("receiver panic");
}
