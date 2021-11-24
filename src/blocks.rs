use std::process::Command;

use crate::BlockFn::{self, External, Internal};

pub fn date() -> String {
    let date = Command::new("date").output().unwrap();
    String::from_utf8(date.stdout).unwrap()
}

pub fn bat() -> &'static str {
    "/home/plaos/.local/scripts/statusbar/sb-bat"
}

pub fn cpu() -> &'static str {
    "/home/plaos/.local/scripts/statusbar/sb-cpu"
}

pub const DELIM: &str = "  ";
pub const BLOCKS: &[(u32, BlockFn)] = &[
    (1800, External(|| "/home/plaos/.local/scripts/statusbar/sb-updates")),
    (5   , External(cpu)),
    (3   , External(|| "/home/plaos/.local/scripts/statusbar/sb-mem")),
    (300 , External(|| "/home/plaos/.local/scripts/statusbar/sb-disk")),
    (30  , External(bat)),
    (1   , Internal(date)),
];
