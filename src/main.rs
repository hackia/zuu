use crate::helpers::{ko, ok, run};
use std::env::args;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub mod helpers;

enum Language {
    Rust,
    Go,
    Unknown,
}
fn check_rust(started: Instant) -> i32 {
    let audit = run(
        "Started",
        "Audit",
        "cargo",
        "audit",
        "Audit no detect errors",
        "Audit detect errors",
        Instant::now(),
    );

    let clippy = run("Started",
        "Clippy",
        "cargo",
        "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates  -D unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -F clippy::cargo",
        "Your code is correct",
        "Your code is incorrect",
        Instant::now());
    let tests = run(
        "Started",
        "Tests",
        "cargo",
        "test --no-fail-fast",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    let checkup = run(
        "Started",
        "Check",
        "cargo",
        "check",
        "Your code is correct",
        "Your code is incorrect",
        Instant::now(),
    );
    let format = run(
        "Started",
        "Format",
        "cargo",
        "fmt --check",
        "Your code is formatted correctness",
        "Your project is bad formatted",
        Instant::now(),
    );

    if format.eq(&0) && checkup.eq(&0) && tests.eq(&0) && clippy.eq(&0) && audit.eq(&0) {
        ok("Your code can be committed", started);
        println!();
        0
    } else {
        ko("Your code contains failures", started);
        println!();
        1
    }
}

fn check_go(started: Instant) -> i32 {
    let verify = run(
        "Started",
        "Verify",
        "go",
        "mod verify",
        "Your code can it's verified successfully",
        "Your project is not valid",
        Instant::now(),
    );
    let build = run(
        "Started",
        "Build",
        "go",
        "build",
        "Your code can be built",
        "Your project cannot be built",
        Instant::now(),
    );
    let tests = run(
        "Started",
        "Test",
        "go",
        "test -v",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    let vet = run(
        "Started",
        "Test",
        "go",
        "vet",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    if vet.eq(&0) && tests.eq(&0) && tests.eq(&0) && verify.eq(&0) && build.eq(&0) {
        ok("Your code can be committed", started);
        println!();
        0
    } else {
        ko("Your code contains failures", started);
        println!();
        1
    }
}

fn check(language: &Language, s: Instant) -> i32 {
    match language {
        Language::Rust => check_rust(Instant::now()),
        Language::Go => check_go(Instant::now()),
        Language::Unknown => {
            ko("Language not supported", s);
            1
        }
    }
}

fn detect() -> Language {
    if Path::new("Cargo.toml").exists() {
        Language::Rust
    } else if Path::new("go.mod").exists() {
        Language::Go
    } else {
        Language::Unknown
    }
}

fn status() {
    if Path::new(".git").exists() {
        println!("\x1b[1;32m    Previous\x1b[0m\n");
        assert!(Command::new("git")
            .arg("log")
            .arg("-1")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        println!("\n\x1b[1;32m     Current\x1b[0m\n");
        assert!(Command::new("git")
            .arg("diff")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
        println!();
    }
}
fn spin(b: &str, data: &str) {
    let i = ["|", "/", "-", "\\", "|"];
    for &x in &i {
        print!("\r\x1b[1;37m {x}\x1b[1;32m  Sleeping\x1b[0m");
        std::io::stdout().flush().expect("a");
        sleep(Duration::from_millis(100));
    }
    std::io::stdout().flush().expect("a");
    print!("\r\x1b[1;37m *\x1b[1;32m   {b} \x1b[1;37m{data}\x1b[0m");
    std::io::stdout().flush().expect("a");
}
fn main() {
    let s = Instant::now();
    let args: Vec<String> = args().collect();
    if args.len().eq(&2) && args.get(1).unwrap().eq("--watch") {
        print!("{}", ansi_escapes::CursorHide);
        ok("Enter in watch mode", s);
        loop {
            let code = check(&detect(), s);
            status();
            if code.eq(&0) {
                for _t in 1..61 {
                    spin("Success", "Your code can be committed");
                }
            } else {
                for _t in 1..61 {
                    spin("Failure", "Your code contains failures");
                }
            }
        }
    }
    let code = check(&detect(), s);
    status();
    exit(code);
}
