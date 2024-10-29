use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metric_names = vec![
        "req.page.0",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "worker.thread.0",
        "worker.thread.1",
    ];
    let metrics = AmapMetrics::new(&metric_names);

    println!("{}", metrics.clone());

    // start N task workers
    for idx in 0..N {
        task_workers(idx, metrics.clone());
    }

    // start M request workers
    for _ in 0..M {
        request_worker(metrics.clone())
    }

    loop {
        println!("{}", metrics.clone());
        thread::sleep(Duration::from_secs(1));
    }
}

fn task_workers(idx: usize, metrics: AmapMetrics) {
    // threads to do some work
    thread::spawn(move || {
        loop {
            metrics.incr(format!("worker.thread.{}", idx))?;
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}

fn request_worker(metrics: AmapMetrics) {
    // requests to do some work
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            let page_no = rng.gen_range(0..M);
            metrics.incr(format!("req.page.{}", page_no))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
