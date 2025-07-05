use insta_cmd::get_cargo_bin;
use std::{ffi::OsStr, path::PathBuf, process::Command};

pub struct Fixture {
    bin_path: PathBuf,
}

impl Fixture {
    pub fn new() -> Self {
        let bin_path = get_cargo_bin("tbll");

        Self { bin_path }
    }

    pub fn cmd<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = Command::new(&self.bin_path);
        command.args(args);
        command
    }
}
