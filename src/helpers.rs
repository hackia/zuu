use std::process::Command;
use std::time::Instant;

pub fn title(eta: &str, task: &str) {
    println!("\n\x1b[1;32m     {eta}\x1b[0m {task}");
}

///
/// # Panics
///
#[must_use]
pub fn exec(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .expect("failed to execute cmd")
        .success()
}
///
/// # Panics
///
/// if the program is not founded or command error
///
pub fn msg(text: &str) {
    println!("\n{text}");
}

pub fn ok(text: &str, started: Instant) {
    println!(
        "    \x1b[1;32mFinished\x1b[0m {text} take {}s",
        started.elapsed().as_secs()
    );
}
pub fn okay(text: &str) {
    println!("    \x1b[1;32mFinished\x1b[0m {text}",);
}

pub fn failure(text: &str) {
    println!("    \x1b[1;31mFinished\x1b[0m {text}",);
}

pub fn ko(text: &str, started: Instant) {
    println!(
        "    \x1b[1;31mFinished\x1b[1;37m {text} take {}s\x1b[0m",
        started.elapsed().as_secs()
    );
}

///
/// # Panics
///
#[must_use]
pub fn project() -> String {
    assert!(exec(
        "sh",
        &[
            "-c",
            "cat Cargo.toml | grep name | cut -d '\"'  -f 2 > app.txt"
        ]
    ));
    std::fs::read_to_string("app.txt").expect("")
}
