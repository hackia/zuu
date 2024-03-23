use std::fs::File;
use std::process::Command;
use std::time::Instant;
pub fn title(eta: &str, task: &str) {
    println!("\n\x1b[1;32m     {eta}\x1b[0m {task}");
}

///
/// # Panics
///
/// if the program is not founded or command error
///
#[must_use]
pub fn run(program: &str, args: &str, f: File, stdout: bool) -> i32 {
    if stdout {
        if Command::new(program)
            .args(args.split_whitespace())
            .current_dir(".")
            .stdout(f)
            .spawn()
            .expect("")
            .wait()
            .unwrap()
            .success()
        {
            0
        } else {
            1
        }
    } else {
        if Command::new(program)
            .args(args.split_whitespace())
            .current_dir(".")
            .stderr(f)
            .spawn()
            .expect("")
            .wait()
            .unwrap()
            .success()
        {
            0
        } else {
            1
        }
    }
}
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
