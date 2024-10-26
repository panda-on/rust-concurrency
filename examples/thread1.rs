use std::sync::mpsc;

use anyhow::{anyhow, Result};
use rand::random;

const NUM_PRODUCER: u8 = 5;
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for idx in 0..NUM_PRODUCER {
        let tx = tx.clone();
        std::thread::spawn(move || producer(idx, tx));
    }

    drop(tx);

    let receiver = std::thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer: done");
        9
    });

    let secret = receiver
        .join()
        .map_err(|e| anyhow!("Failed to join thread: {:?}", e))?;
    println!("secret: {}", secret);

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    prd_idx: usize,
    data: u32,
}

fn producer(idx: u8, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let data = random::<u32>();
        tx.send(Msg::new(idx as usize, data))?;
        let sleep_time = random::<u8>() as u64 * 10;
        std::thread::sleep(std::time::Duration::from_millis(sleep_time));
        if sleep_time % 10 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    Ok(())
}

impl Msg {
    fn new(idx: usize, data: u32) -> Self {
        Self { prd_idx: idx, data }
    }
}
