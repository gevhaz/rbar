mod config;

use std::process::Command;
use std::sync::mpsc::{RecvError, channel};
use std::thread;
use std::time::Duration;

use crate::config::{BLOCKS, DELIM};

fn main() {
    // If the `run` function errors, print the error and exit.
    if let Err(e) = run() {
        eprintln!("{}", e.to_string());
        std::process::exit(1);
    }
}

/// The signature of a block. Anything with this signature
/// can be used as a block.
pub type Procedure = fn() -> String;

/// A Block is the combination of an interval and a procedure,
/// evaluating the <procedure> each <interval> seconds.
pub type Block = (Duration, Procedure);

/// Internal representation of something failable.
type Result<T> = std::result::Result<T, RecvError>;

/// Spawn each block in its own thread, and wait for them to update.
fn run() -> Result<()> {
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
            thread::sleep(interval);
        });
    }

    loop {
        // Wait for any block to be updated
        let (id, msg) = rx.recv()?;
        results[id] = msg;

        // The "statusbar" is the combination of all blocks, with
        // the specified delimiter between each
        let statusbar = results.join(DELIM);

        // Update the status bar
        let _ = Command::new("xsetroot")
            .arg("-name")
            .arg(statusbar)
            .status();
    }
}
