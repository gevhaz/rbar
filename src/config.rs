use std::fs;
use std::process::Command;

type IntervalSeconds = u32;
type Procedure = fn() -> String;
type Block = (IntervalSeconds, Procedure);

/// This defines what blocks should be drawn, and in what order.
pub const BLOCKS: &[Block] = &[(30, bat), (1, date)];

/// This defines the delimiter between each block. Use empty
/// string for no delimiter.
pub const DELIM: &str = "  ";

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

fn date() -> String {
    run_cmd("date", &["+%a %b %d %H:%M:%S"], &[("LC_ALL", "en")])
}

fn bat() -> String {
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
