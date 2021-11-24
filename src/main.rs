mod blocks;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crate::blocks::BLOCKS;

#[derive(Copy, Clone)]
pub enum BlockFn {
    Internal(fn() -> String),
    External(fn() -> &'static str),
}

impl BlockFn {
    fn resolve(&self) -> String {
        match self {
            BlockFn::Internal(f) => f(),
            BlockFn::External(path) => {
                String::from_utf8(Command::new(path()).output().unwrap().stdout).unwrap()
            }
        }
    }
}

const DELIM: &str = " | ";

fn main() {
    let mut results = BLOCKS
        .into_iter()
        .map(|(_, block)| block.resolve())
        .collect::<Vec<_>>();

    let (tx, rx) = channel();

    for (id, &(interval, proc)) in BLOCKS.iter().enumerate() {
        let thread_tx = tx.clone();

        thread::spawn(move || loop {
            let _ = thread_tx.send((id, proc.resolve()));
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
