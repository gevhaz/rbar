mod config;

use std::process::Command;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use crate::config::{BLOCKS, DELIM};

/// How many seconds to sleep before evaluating again.
pub type IntervalSeconds = u64;
/// The signature of a block. Anything with this signature
/// can be used as a block.
pub type Procedure = fn() -> String;
/// A Block is the combination of an interval and a procedure,
/// evaluating the <procedure> each <interval> seconds.
pub type Block = (IntervalSeconds, Procedure);

fn main() {
    // Evaluate all blocks once on startup
    let mut results: Vec<String> = BLOCKS.iter().map(|(_, block)| block()).collect();

    // Create sender and reveiver for communicating between threads
    let (tx, rx) = channel();

    for (id, &(interval, proc)) in BLOCKS.iter().enumerate() {
        // Clone the sender, in order to use it from another thread
        let thread_tx = tx.clone();

        // Create a new thread for the current block
        thread::spawn(move || loop {
            // Evaluate the block and send it to the receiver
            let _ = thread_tx.send((id, proc()));

            // Sleep for the specified interval
            thread::sleep(Duration::from_secs(interval));
        });
    }

    loop {
        // Wait for any block to be updated
        let (id, msg) = rx.recv().unwrap();
        results[id] = msg;

        // Update the status bar
        let _ = Command::new("xsetroot")
            .arg("-name")
            .arg(results.join(DELIM))
            .status();
    }
}
