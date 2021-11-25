mod config;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crate::config::{BLOCKS, DELIM};

pub type IntervalSeconds = u64;
pub type Procedure = fn() -> String;
pub type Block = (IntervalSeconds, Procedure);

fn main() {
    let mut results: Vec<String> = BLOCKS.iter().map(|(_, block)| block()).collect();

    let (tx, rx) = channel();

    for (id, &(interval, proc)) in BLOCKS.iter().enumerate() {
        let thread_tx = tx.clone();

        thread::spawn(move || loop {
            let _ = thread_tx.send((id, proc()));
            thread::sleep(Duration::from_secs(interval));
        });
    }

    loop {
        let (id, msg) = rx.recv().unwrap();
        results[id] = msg;

        let _ = Command::new("xsetroot")
            .arg("-name")
            .arg(results.join(DELIM))
            .status();
    }
}
