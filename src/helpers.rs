use std::process::Command;
use std::time::Instant;
pub fn title(text: &str) {
    println!("\n\x1b[1;37m{text}\x1b[0m\n");
}

///
/// # Panics
///
/// if the program is not founded or command error
///
pub fn run(t: &str, program: &str, args: &str, s: &str, e: &str, x: Instant) {
    title(t);
    assert!(
        Command::new(program)
            .args(args.split_whitespace())
            .current_dir(".")
            .spawn()
            .expect("")
            .wait()
            .unwrap()
            .success(),
        "{}",
        e
    );
    ok(s, x);
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
