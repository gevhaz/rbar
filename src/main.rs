mod blocks;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

const DELIM: &str = " | ";

type Block = (u32, fn() -> String);

fn main() {
    let blocks: &[Block] = &[
        (3000, blocks::date),
    ];

    let mut results = blocks
        .into_iter()
        .map(|(_, block)| block())
        .collect::<Vec<_>>();

    let (tx, rx) = channel();

    for (id, &(interval, proc)) in blocks.iter().enumerate() {
        let thread_tx = tx.clone();

        thread::spawn(move || loop {
            let _ = thread_tx.send((id, proc()));
            thread::sleep(Duration::from_millis(interval as u64));
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
