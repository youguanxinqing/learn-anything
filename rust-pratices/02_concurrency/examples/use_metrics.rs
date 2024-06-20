use std::{thread, time::Duration};

use anyhow;
use concurrency::Metrics;
use rand::{thread_rng, Rng};

const N: i32 = 2;
const M: i32 = 4;

fn main() -> anyhow::Result<()> {
    let mut metrics = Metrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }
    for idx in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{:?}", metrics.snapshot());
    }

    Ok(())
}

fn task_worker(idx: i32, mut metrics: Metrics) {
    thread::spawn(move || loop {
        // sleep
        let mut rng = thread_rng();
        let hold_time: u64 = rng.gen_range(0..5);
        thread::sleep(Duration::from_secs(hold_time));

        metrics.inc(format!("/api/login/v1/{}", idx)).unwrap();
    });
}

fn request_worker(mut metrics: Metrics) {
    thread::spawn(move || {
        let mut rng = thread_rng();
        let hold_time: u64 = rng.gen_range(0..3);
        thread::sleep(Duration::from_secs(hold_time));

        metrics.inc("/api/request/v1").unwrap();
    });
}
