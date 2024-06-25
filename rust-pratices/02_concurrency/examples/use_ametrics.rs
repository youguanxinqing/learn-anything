use std::thread;
use std::time::Duration;

use concurrency::metrics::AmapMetrics;
use concurrency::metrics::ametrics::{create_worker, move_worker};



fn main() {
    let m = AmapMetrics::new(&[
        "create/api/v1/0",
        "create/api/v1/1",
        "move/api/v1",
    ]);

    // for create
    for idx in 0..2 {
        let _ = create_worker(idx, m.clone());
    }

    // for move
    for _ in 0..4 {
        let _ = move_worker(m.clone());
    }

    // print
    loop {
        println!("{}", m);
        thread::sleep(Duration::from_secs(5));
    }
}
