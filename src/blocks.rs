use std::process::Command;

pub fn date() -> String {
    let date = Command::new("date").output().unwrap();
    String::from_utf8(date.stdout).unwrap()
}
