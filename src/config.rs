use std::process::Command;

use crate::Block;

/// This defines what blocks should be drawn, and in what order.
pub const BLOCKS: &[Block] = &[
    (10, blocks::mem),
    (10, blocks::cpu),
    (120, blocks::disk),
    (30, blocks::bat),
    (1, blocks::date),
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

/// Given the basename of a script, use a pre-defined directory
/// and execute the script in it.
fn run_script(script_name: &str) -> String {
    let base = String::from("/home/plaos/.local/scripts/statusbar");
    let script = format!("{}/{}", base, script_name);
    run_cmd(&script, &[], &[])
}

mod blocks {
    use std::fs;

    use super::{run_cmd, run_script};

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

    /// Displays CPU information; percentage used and temperature.
    pub fn cpu() -> String {
        run_script("sb-cpu")
    }

    /// Block for displaying the current date and time.
    pub fn date() -> String {
        run_cmd("date", &["+%a %b %d %H:%M"], &[("LC_ALL", "en")])
    }

    /// Displays the amount of available storage left.
    pub fn disk() -> String {
        run_script("sb-disk")
    }

    /// Displays a percentage of the used memory.
    pub fn mem() -> String {
        run_script("sb-mem")
    }
}
