use std::process::{exit, Command, ExitCode};

fn main() -> ExitCode {
    exit(
        Command::new("cargo")
            .arg("install")
            .arg("zuu")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .code()
            .unwrap(),
    );
}
