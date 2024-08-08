use std::fs::{create_dir_all, read_to_string};
use std::io::{stdout, Stdout, Write};
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::thread::sleep;
use std::time::Duration;
use std::{fs::File, io::Error, process::Command};

const RUST_TASKS: [&str;5] = [
    "deny check",
    "audit",
    "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms -F rust-2021-compatibility -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::todo -F clippy::needless_borrow -F clippy::use_self -F clippy::redundant_clone -F clippy::manual_memcpy -F clippy::manual_assert -F clippy::single_match_else -F clippy::unwrap_used -F clippy::expect_used -F clippy::panic -F clippy::complexity -D clippy::cargo -F keyword_idents -F warnings -F let-underscore",
    "test -j 4 --no-fail-fast",
    "fmt --check"];

const ANIMATE: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];
fn ok(task: &str) -> Result<(), Error> {
    animate(250, true, task)
}
fn ko(task: &str) -> Result<(), Error> {
    animate(100, false, task)
}

fn animate(t: u64, success: bool, task: &str) -> Result<(), Error> {
    let mut out: Stdout = stdout();
    let final_char = if success { "OK" } else { "KO" };
    for x in &ANIMATE {
        write!(out, "\r\x1B[K")?;
        out.flush()?;
        write!(out, "\r\x1b[1;37m[ {x}{x} ] {task}\x1b[0m")?;
        out.flush()?;
        sleep(Duration::from_millis(t));
    }
    write!(out, "\r\x1B[K")?;
    out.flush()?;
    if success {
        write!(
            out,
            "\r\x1b[1;37m[ \x1b[1;32m{final_char}\x1b[1;37m ] {task}\x1b[0m\n"
        )?;
        out.flush()?;
        return Ok(());
    }
    eprintln!(
        "\x1b[1;37m[ \x1b[1;31m{final_char}\x1b[1;37m ] {task}\n\n\x1b[1;90m{}\x1b[0;30m",
        read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{task}").as_str()
        )?
    );
    sleep(Duration::from_millis(t));
    Err(Error::last_os_error())
}

fn msg(s: &str, e: &str, success: bool) -> Result<(), Error> {
    let mut out: Stdout = stdout();
    if success {
        write!(out, "\r\x1B[K")?;
        out.flush()?;
        write!(out, "\r\x1b[1;37m[ \x1b[1;32mOK\x1b[1;37m ] {s}\x1b[0m\n")?;
        out.flush()?;
        enable_cursor();
        return Ok(());
    }
    write!(out, "\r\x1B[K")?;
    out.flush()?;
    write!(out, "\r\x1b[1;37m[\x1b[1;31mKO\x1b[1;37m ] {e}\x1b[0m\n")?;
    out.flush()?;
    enable_cursor();
    Err(Error::last_os_error())
}
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    assert!(stdout().flush().is_ok());
}
fn disable_cursor() {
    print!("\x1b[?25l");
    assert!(stdout().flush().is_ok());
}
fn enable_cursor() {
    print!("\x1b[?25h");
    assert!(stdout().flush().is_ok());
}
fn zuu() -> Result<(), Error> {
    if !Path::new("zuu").exists() {
        create_dir_all("zuu")?;
        create_dir_all(format!("zuu{MAIN_SEPARATOR_STR}stderr"))?;
        create_dir_all(format!("zuu{MAIN_SEPARATOR_STR}stdout"))?;
    }
    for x in RUST_TASKS {
        let p: Vec<&str> = x.split_whitespace().collect();

        if Command::new("cargo")
            .stdout(File::create(format!(
                "zuu{MAIN_SEPARATOR_STR}stdout{MAIN_SEPARATOR_STR}{}",
                p[0]
            ))?)
            .stderr(File::create(format!(
                "zuu{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{}",
                p[0]
            ))?)
            .args(x.split_whitespace())
            .current_dir(".")
            .spawn()?
            .wait()?
            .success()
        {
            if let Some(o) = p.first() {
                assert!(ok(o).is_ok());
            }
            continue;
        }
        if let Some(o) = p.first() {
            return ko(o);
        }
        return Err(Error::last_os_error());
    }
    Ok(())
}
fn main() -> Result<(), Error> {
    disable_cursor();
    clear();
    msg(
        "Code can be committed",
        "Code can't be committed",
        zuu().is_ok(),
    )
}
