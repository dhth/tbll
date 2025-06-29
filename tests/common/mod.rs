use insta_cmd::get_cargo_bin;
use std::process::Command;

pub fn base_command() -> Command {
    Command::new(get_cargo_bin("tbll"))
}
