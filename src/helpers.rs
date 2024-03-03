use cvt::cvt;
use gettextrs::gettext;
use libc::{ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::ffi::c_ushort;
use std::mem;
use std::process::Command;
use std::time::Instant;

pub fn title(text: &str) {
    println!("\n{}", banner(text, "="));
}

#[repr(C)]
struct TermSize {
    row: c_ushort,
    col: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

///
/// # Panics
///
/// if the program is not founded or command error
///
pub fn run(program: &str, args: &str, s: &str, e: &str, x: Instant) {
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

fn center(text: &str) -> String {
    format!(
        "{}{}",
        String::from(' ').repeat((size().0 / 2) - text.to_string().len()),
        text
    )
}
fn banner(text: &str, b: &str) -> String {
    let banner = String::from(b).repeat(size().0);
    format!("{}\n{}\n{}\n", banner, center(text), banner)
}

pub fn msg(text: &str) {
    println!("\n{}", banner(text, "-"));
}

pub fn ok(text: &str, started: Instant) {
    println!(
        "    \x1b[1;32m{}\x1b[0m {} {} {}s",
        gettext("Finished"),
        text,
        gettext("verified in").as_str(),
        started.elapsed().as_secs()
    );
}

pub fn ko(text: &str, started: Instant) {
    println!(
        "    \x1b[1;31m{}\x1b[0m {} {} {}s",
        gettext("Finished"),
        text,
        gettext("take").as_str(),
        started.elapsed().as_secs()
    );
}

///
/// # Panics
/// aa
#[must_use]
pub fn size() -> (usize, usize) {
    unsafe {
        let mut size: TermSize = mem::zeroed();
        assert!(cvt(ioctl(
            STDOUT_FILENO,
            TIOCGWINSZ,
            std::ptr::addr_of_mut!(size)
        ))
        .is_ok());
        (size.col as usize, size.row as usize)
    }
}
