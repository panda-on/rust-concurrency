use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rand::Rng;

fn main() {
    let counter = Arc::new(Mutex::new(HashMap::new()));

    (0..4).for_each(|idx| {
        let cur_cnt = counter.clone();
        thread::spawn(move || {
            (0..10).for_each(|_| {
                let mut data = cur_cnt.lock().unwrap();
                let thread_name = format!("thread.{}", idx);
                let value = data.entry(thread_name).or_insert(0);
                *value += 1;
                let mut rng = rand::thread_rng();
                thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            });
        });
    });

    loop {
        println!("{:?}", counter.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
