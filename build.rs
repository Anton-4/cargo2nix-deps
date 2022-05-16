use std::process::{Command};

fn main() {
    let exit_status = Command::new("zig")
            .arg("version")
            .output()
            .unwrap()
            .status;

    assert!(exit_status.success())
}