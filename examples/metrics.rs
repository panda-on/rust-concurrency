use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;

const N: usize = 4;
const M: usize = 8;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    println!("{:?}", metrics.snapshot());

    // start N task workers
    for idx in 0..N {
        task_workers(idx, metrics.clone());
    }

    // start M request workers
    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        println!("{:?}", metrics.snapshot());
        thread::sleep(Duration::from_secs(1));
    }
}

fn task_workers(idx: usize, metrics: Metrics) {
    // threads to do some work
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics
            .incr(format!("worker.thread.{}", idx))
            .expect("task thread failed");
    });
}

fn request_worker(metrics: Metrics) {
    // requests to do some work
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        let page_no = rng.gen_range(1..10);
        metrics
            .incr(format!("req.page.{}", page_no))
            .expect("request thread failed");
    });
}
