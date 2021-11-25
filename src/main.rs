mod blocks;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crate::blocks::{BLOCKS, DELIM};

fn main() {
    let mut results = BLOCKS.iter().map(|(_, block)| block()).collect::<Vec<_>>();

    let (tx, rx) = channel();

    for (id, &(interval, proc)) in BLOCKS.iter().enumerate() {
        let thread_tx = tx.clone();

        thread::spawn(move || loop {
            let _ = thread_tx.send((id, proc()));
            thread::sleep(Duration::from_secs(interval as u64));
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
