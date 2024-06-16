use std::time::Instant;

pub fn title(eta: &str, task: &str) {
    println!("\n\x1b[1;32m     {eta}\x1b[0m {task}");
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
