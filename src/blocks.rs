use crate::BlockFn;
use std::process::Command;

pub fn date() -> BlockFn {
    BlockFn::Function(|| {
        let date = Command::new("date").output().unwrap();
        String::from_utf8(date.stdout).unwrap()
    })
}

pub fn bat() -> BlockFn {
    BlockFn::Script("/home/plaos/.local/scripts/statusbar/sb-bat")
}
