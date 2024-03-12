use crate::helpers::{ko, ok, okay, run};
use std::env::args;
use std::fs;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub mod helpers;

enum Language {
    Rust,
    Go,
    Docker,
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

fn docker(s: Instant) -> i32 {
    let x = run(
        "Started",
        "Docker",
        "docker-compose",
        "up",
        "Your code can be committed",
        "Your code contains failures",
        s,
    );
    let _ = run(
        "Closing",
        "Docker",
        "docker-compose",
        "down",
        "Your code can be committed",
        "Your code contains failures",
        s,
    );

    match x {
        0 => 0,
        _ => 1,
    }
}

fn check(language: &Language, s: Instant) -> i32 {
    match language {
        Language::Rust => check_rust(s),
        Language::Go => check_go(s),
        Language::Unknown => {
            ko("Language not supported", s);
            1
        }
        Language::Docker => docker(s),
    }
}

fn detect() -> Language {
    if Path::new("compose.yaml").exists() {
        Language::Docker
    } else if Path::new("Cargo.toml").exists() {
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
        let _ = Command::new("git")
            .arg("log")
            .arg("-1")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
        println!("\n\x1b[1;32m     Current\x1b[0m\n");
        let _ = Command::new("git")
            .arg("diff")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
        println!();
    }
    if Path::new(".hg").exists() {
        println!("\x1b[1;32m    Previous\x1b[0m\n");
        let _ = Command::new("hg")
            .arg("log")
            .arg("-l")
            .arg("1")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
        println!("\n\x1b[1;32m     Current\x1b[0m\n");
        let _ = Command::new("hg")
            .arg("diff")
            .arg("--stat")
            .current_dir(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success();
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

fn init() {
    let git_hook_content = "#!/bin/sh\nzuu\n exit $?";
    let hg_hook_content = "[hooks]\nprecommit = zuu";
    if Path::new(".git").exists() {
        let mut f = fs::File::create(
            format!(".git{MAIN_SEPARATOR}hooks{MAIN_SEPARATOR}pre-commit").as_str(),
        )
        .expect("failed to create the hook file");
        f.write_all(git_hook_content.as_bytes())
            .expect("failed to write content");
        f.sync_all().expect("failed to sync data");
    }
    if Path::new(".hg").exists() {
        let mut f = fs::File::create(format!(".hg{MAIN_SEPARATOR}hgrc").as_str())
            .expect("failed to create the hook file");
        f.write_all(hg_hook_content.as_bytes())
            .expect("failed to write content");
        f.sync_all().expect("failed to sync data");
    }
}

fn watch(s: Instant) {
    print!("{}", ansi_escapes::CursorHide);
    ok("Enter in watch mode", s);
    loop {
        let code = check(&detect(), s);
        status();
        okay("Press Ctrl+c to quit");
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

fn main() {
    let s = Instant::now();
    let args: Vec<String> = args().collect();
    if args.len().eq(&2) && args.get(1).unwrap().eq("init") {
        init();
        okay("Your project it's now tracked by zuu");
        exit(0);
    }
    if args.len().eq(&2) && args.get(1).unwrap().eq("--watch") {
        watch(s);
    }
    let code = check(&detect(), s);
    status();
    exit(code);
}
