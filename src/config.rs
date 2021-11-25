use std::process::Command;

use crate::Block;

/// This defines what blocks should be drawn, and in what order.
pub const BLOCKS: &[Block] = &[(30, blocks::bat), (1, blocks::date)];

/// This defines the delimiter between each block. Use empty
/// string for no delimiter.
pub const DELIM: &str = "  ";

/// Helper function to improve ergonomics when executing external commands.
fn run_cmd(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> String {
    let mut command = Command::new(cmd);
    command.args(args);

    for &(k, v) in envs {
        command.env(k, v);
    }

    String::from_utf8(command.output().unwrap().stdout)
        .unwrap()
        .trim_end()
        .into()
}

mod blocks {
    use std::fs;

    use super::run_cmd;

    /// Block for displaying the current date and time.
    pub fn date() -> String {
        run_cmd("date", &["+%a %b %d %H:%M:%S"], &[("LC_ALL", "en")])
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

        format!("[{}{}%]", status, cap)
    }
}
