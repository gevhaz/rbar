use std::process::Command;

use crate::BlockFn::{self, Internal, External};

pub fn date() -> String {
    let date = Command::new("date").output().unwrap();
    String::from_utf8(date.stdout).unwrap()
}

pub fn bat() -> &'static str {
    "/home/plaos/.local/scripts/statusbar/sb-bat"
}

type Block = (u32, BlockFn);

pub const BLOCKS: &[Block] = &[
    (3000, Internal(date)),
    (1000, External(bat)),
];
