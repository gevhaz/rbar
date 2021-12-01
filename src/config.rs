use std::process::Command;
use std::time::Duration;

use crate::Block;

/// This defines what blocks should be drawn, and in what order.
pub const BLOCKS: &[Block] = &[
    (Duration::from_secs(1800), blocks::updates),
    (Duration::from_secs(5), blocks::cpu),
    (Duration::from_secs(5), blocks::mem),
    (Duration::from_secs(60), blocks::disk),
    (Duration::from_secs(0), blocks::audio),
    (Duration::from_secs(0), blocks::vpn),
    (Duration::from_secs(1), blocks::net),
    (Duration::from_secs(30), blocks::bat),
    (Duration::from_secs(1), blocks::date),
];

/// This defines the delimiter between each block. Use empty
/// string for no delimiter.
pub const DELIM: &str = "  ";

/// Helper function to improve ergonomics when executing external commands.
/// This can be useful, for example, when you want to use scripts as blocks
/// in the status bar.
fn run_cmd(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> String {
    String::from_utf8(
        Command::new(cmd)
            .args(args)
            .envs(Vec::from(envs))
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim_end()
    .into()
}

fn run_script(name: &str) -> String {
    let path = format!("/home/plaos/.local/scripts/statusbar/{}", name);
    run_cmd(&path, &[], &[])
}

mod blocks {
    use std::fs;

    use super::{run_cmd, run_script};

    /// Block for displaying the current date and time.
    pub fn date() -> String {
        run_cmd("date", &["+%a %b %d %H:%M:%S"], &[("LC_ALL", "en")])
    }

    pub fn cpu() -> String {
        run_script("sb-cpu")
    }

    pub fn mem() -> String {
        run_script("sb-mem")
    }

    pub fn disk() -> String {
        run_script("sb-disk")
    }

    pub fn audio() -> String {
        run_script("sb-audio")
    }

    pub fn vpn() -> String {
        run_script("sb-vpn")
    }

    pub fn net() -> String {
        run_script("sb-net")
    }

    pub fn updates() -> String {
        run_script("sb-updates")
    }

    /// Block for displaying the current battery level, as well
    /// as if it is charging or not.
    pub fn bat() -> String {
        let cap = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
            .unwrap()
            .trim_end()
            .to_string();

        let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")
            .unwrap()
            .trim_end()
            .to_string();

        let status = match status.as_ref() {
            "Charging" => "+",
            "Discharging" | "Full" => "",
            _ => "?",
        };

        format!("{}{}%", status, cap)
    }
}
