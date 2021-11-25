use std::fs;
use std::process::Command;

type IntervalSeconds = u32;
type Procedure = fn() -> String;
type Block = (IntervalSeconds, Procedure);

pub const DELIM: &str = "  ";
pub const BLOCKS: &[Block] = &[(30, bat), (1, date)];

fn run_cmd(cmd: &str, args: &[&str], envs: &[(&str, &str)]) -> String {
    let mut command = Command::new(cmd);
    command.args(args);

    for (k, v) in envs {
        command.env(k, v);
    }

    String::from_utf8(command.output().unwrap().stdout)
        .unwrap()
        .trim_end()
        .into()
}

pub fn date() -> String {
    run_cmd("date", &["+%a %b %d %H:%M:%S"], &[("LC_ALL", "en")])
}

pub fn bat() -> String {
    let cap: String = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .unwrap()
        .trim_end()
        .into();
    let status: String = fs::read_to_string("/sys/class/power_supply/BAT0/status")
        .unwrap()
        .trim_end()
        .into();

    // let cap = run_cmd("cat", &["/sys/class/power_supply/BAT0/capacity"], &[]);
    // let status = run_cmd("cat", &["/sys/class/power_supply/BAT0/status"], &[]);

    let status = match status.as_ref() {
        "Charging" => "+",
        "Discharging" | "Full" => "",
        _ => "?",
    };
    format!("[{}{}%]", status, cap)
}
