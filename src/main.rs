use crate::helpers::{ko, ok, run};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

pub mod helpers;

fn check_rust(started: Instant) {
    let audit = Instant::now();
    run(
        "Started",
        "Audit",
        "cargo",
        "audit",
        "Audit no detect errors",
        "Audit detect errors",
        audit,
    );
    let clippy = Instant::now();
    run("Started",
        "Clippy",
        "cargo",
        "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates  -D unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -F clippy::cargo",
        "Your code is correct",
        "Your code is incorrect",
        clippy);
    let test = Instant::now();
    run(
        "Started",
        "Tests",
        "cargo",
        "test --no-fail-fast",
        "No test failures",
        "Test have failures",
        test,
    );
    let check = Instant::now();
    run(
        "Started",
        "Check",
        "cargo",
        "check",
        "Your code is correct",
        "Your code is incorrect",
        check,
    );
    let fmt = Instant::now();
    run(
        "Started",
        "Format",
        "cargo",
        "fmt --check",
        "Your code is formatted correctness",
        "Your project is bad formatted",
        fmt,
    );
    ok("Your code can be committed", started);
    println!();
}

fn main() {
    let s = Instant::now();
    if Path::new("Cargo.toml").exists() {
        check_rust(s);
        exit(0);
    }
    ko("Source code not supported", s);
    exit(1);
}
