use crate::helpers::{ko, ok, okay, run};
use crate::Language::{Cmake, Composer, Docker, Go, Make, Rust, Unknown};
use std::collections::HashMap;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::{read_to_string, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

pub mod helpers;

enum Language {
    Rust,
    Go,
    Docker,
    Make,
    Composer,
    Cmake,
    Unknown,
}

fn make(started: Instant, cmd: &str) -> i32 {
    run(
        "Started",
        cmd,
        "make",
        cmd,
        format!("make {cmd} executed successfully").as_str(),
        format!("make {cmd} executed failure").as_str(),
        started,
    )
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(File::open(filename).expect("failed to found"))
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn check_make(started: Instant) -> i32 {
    let mut cmd: Vec<String> = Vec::new();
    for x in &read_lines("Makefile") {
        if x.starts_with("all") {
            cmd.push("all".to_string());
        }
        if x.starts_with("install") {
            cmd.push("install".to_string());
        }
        if x.starts_with("dist") {
            cmd.push("dist".to_string());
        }
        if x.starts_with("clean") {
            cmd.push("clean".to_string());
        }

        if x.starts_with("uninstall") {
            cmd.push("uninstall".to_string());
        }
        if x.starts_with("install") {
            cmd.push("install".to_string());
        }

        if x.starts_with("install-strip") {
            cmd.push("install-strip".to_string());
        }
        if x.starts_with("distclean") {
            cmd.push("distclean".to_string());
        }

        if x.starts_with("install") {
            cmd.push("install".to_string());
        }

        if x.starts_with("maintainer-clean") {
            cmd.push("maintainer-clean".to_string());
        }
        if x.starts_with("mostlyclean") {
            cmd.push("mostlyclean".to_string());
        }
    }
    for x in &cmd {
        assert!(make(started, x.as_str()).eq(&0));
    }
    0
}

fn check_cmake(started: Instant) -> i32 {
    if run(
        "Started",
        "Cmake",
        "cmake",
        ".",
        "Makefile created successfully",
        "Failed to create Makefile",
        started,
    )
    .eq(&0)
        && run(
            "Started",
            "Make",
            "make",
            "",
            "Project built successfully",
            "Failed to built the project",
            started,
        )
        .eq(&0)
        && run(
            "Started",
            "Install",
            "make",
            "install",
            "Project installed successfully",
            "Failed to install the project",
            started,
        )
        .eq(&0)
    {
        return 0;
    }
    1
}

fn check_rust(started: Instant) -> i32 {
    let project = run(
        "Started",
        "Project",
        "cargo",
        "verify-project",
        "verify-project no detect errors",
        "verify-project detect errors",
        Instant::now(),
    );
    let bench = run(
        "Started",
        "Project",
        "cargo",
        "bench --no-fail-fast --all-targets --message-format human -j 4",
        "bench no detect errors",
        "bench detect errors",
        Instant::now(),
    );
    let build = run(
        "Started",
        "Build",
        "cargo",
        "build --all-targets --release -j 4 --future-incompat-report",
        "Build no detect errors",
        "Build detect errors",
        Instant::now(),
    );
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
                     "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -D clippy::cargo",
                     "Your code is correct",
                     "Your code is incorrect",
                     Instant::now());
    let tests = run(
        "Started",
        "Tests",
        "cargo",
        "test --all-targets --all-features --release -j 4 --no-fail-fast",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    let checkup = run(
        "Started",
        "Check",
        "cargo",
        "check --all-targets --release --message-format human -j 4",
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
    let install = run(
        "Started",
        "Install",
        "cargo",
        "install --path . --no-track --bins --examples --all-features -j 4 --force",
        "Your project can be installed",
        "Your project cannot be installed",
        Instant::now(),
    );
    if format.eq(&0)
        && checkup.eq(&0)
        && tests.eq(&0)
        && clippy.eq(&0)
        && audit.eq(&0)
        && project.eq(&0)
        && bench.eq(&0)
        && build.eq(&0)
        && install.eq(&0)
    {
        ok("Your code can be committed", started);
        status();
        0
    } else {
        ko("Your code contains failures", started);
        status();
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
        status();
        0
    } else {
        ko("Your code contains failures", started);
        status();
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
        Rust => check_rust(s),
        Go => check_go(s),
        Docker => docker(s),
        Make => check_make(s),
        Composer => check_composer(s),
        Cmake => check_cmake(s),
        Unknown => {
            ko("Language not supported", s);
            1
        }
    }
}

fn detect() -> &'static Language {
    for (f, l) in &all() {
        if Path::new(f.as_str()).exists() {
            return l;
        }
    }
    &Unknown
}

fn php(started: Instant, f: &str, t: &str, command: &str) -> i32 {
    if Path::new(f).exists() {
        return run(
            "Started",
            t,
            "php",
            command,
            "All tests passes",
            "Test have failures",
            started,
        );
    }
    1
}
fn check_composer(started: Instant) -> i32 {
    assert!(run(
        "Started",
        "Install",
        "composer",
        "install",
        "Project can be installed",
        "Project install failed",
        started
    )
    .eq(&0));
    let audit = run(
        "Started",
        "Audit",
        "composer",
        "audit",
        "Audit no detect error",
        "Audit detect errors",
        started,
    );
    let diagnose = run(
        "Started",
        "Diagnose",
        "composer",
        "diagnose",
        "Audit no detect error",
        "Diagnose detect errors",
        started,
    );
    let tests = php(
        started,
        "phpunit.xml",
        "Phpunit",
        format!("vendor{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}phpunit").as_str(),
    );
    let checkup = php(
        started,
        "phpstan.neon",
        "Phpstan",
        format!("vendor{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}phpstan").as_str(),
    );

    if tests.eq(&0) && checkup.eq(&0) && audit.eq(&0) && diagnose.eq(&0) {
        return 0;
    }
    1
}
fn all() -> HashMap<String, &'static Language> {
    let mut all: HashMap<String, &Language> = HashMap::new();
    assert!(all.insert(String::from("compose.yaml"), &Docker).is_none());
    assert!(all.insert(String::from("Cargo.toml"), &Rust).is_none());
    assert!(all.insert(String::from("go.mod"), &Go).is_none());
    assert!(all.insert(String::from("Makefile"), &Make).is_none());
    assert!(all
        .insert(String::from("composer.json"), &Composer)
        .is_none());
    assert!(all.insert(String::from("CMakeLists.txt"), &Cmake).is_none());
    all
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
        okay("Don't forget to add the execution right for the .git/hooks/pre-commit file");
    }
    if Path::new(".hg").exists() {
        let mut f = fs::File::create(format!(".hg{MAIN_SEPARATOR}hgrc").as_str())
            .expect("failed to create the hook file");
        f.write_all(hg_hook_content.as_bytes())
            .expect("failed to write content");
        f.sync_all().expect("failed to sync data");
    }
}

fn waiting(code: i32) {
    print!("{}", ansi_escapes::CursorHide);
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

fn watch(s: Instant) {
    print!("{}", ansi_escapes::CursorHide);
    ok("Enter in watch mode", s);
    loop {
        waiting(check(detect(), s));
    }
}

fn main() {
    print!("{}", ansi_escapes::CursorHide);
    let s = Instant::now();
    let args: Vec<String> = args().collect();
    if args.len().eq(&2) && args.get(1).unwrap().eq("init") {
        init();
        okay("Your project it's now tracked by zuu");
        exit(0);
    }
    if args.contains(&"--watch".to_string()) {
        watch(s);
    }
    let code = check(detect(), s);
    status();
    print!("{}", ansi_escapes::CursorShow);
    exit(code);
}
