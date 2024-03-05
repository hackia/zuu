use std::process::{exit, Command};
use std::time::Instant;
pub fn title(eta: &str, task: &str) {
    println!("\n\x1b[1;32m     {eta}\x1b[0m {task}");
}

///
/// # Panics
///
/// if the program is not founded or command error
///
pub fn run(eta: &str, task: &str, program: &str, args: &str, s: &str, e: &str, x: Instant) {
    title(eta, task);
    if Command::new(program)
        .args(args.split_whitespace())
        .current_dir(".")
        .spawn()
        .expect("")
        .wait()
        .unwrap()
        .success()
    {
        ok(s, x);
    } else {
        ko(e, x);
        exit(1);
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

pub fn ko(text: &str, started: Instant) {
    println!(
        "    \x1b[1;31mFinished\x1b[1;37m {text} take {}s\x1b[0m",
        started.elapsed().as_secs()
    );
}
