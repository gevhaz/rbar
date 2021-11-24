mod blocks;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

const DELIM: &str = " | ";

type IntervalMs = u32;

#[derive(Copy, Clone)]
pub enum BlockFn {
    Function(fn() -> String),
    Script(&'static str),
}

impl BlockFn {
    fn resolve(&self) -> String {
        match self {
            BlockFn::Function(f) => f(),
            BlockFn::Script(path) => {
                String::from_utf8(Command::new(path).output().unwrap().stdout).unwrap()
            }
        }
    }
}

type Block = (IntervalMs, BlockFn);

fn main() {
    let blocks: &[Block] = &[(3000, blocks::date()), (1000, blocks::bat())];

    let mut results = blocks
        .into_iter()
        .map(|(_, block)| block.resolve())
        .collect::<Vec<_>>();

    let (tx, rx) = channel();

    for (id, &(interval, proc)) in blocks.iter().enumerate() {
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
