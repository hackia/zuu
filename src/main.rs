use crate::helpers::okay;
use crate::Language::{Rust, Unknown};
use std::collections::HashMap;
use std::path::Path;

pub mod helpers;

use std::process::{exit, Command, ExitCode};

const RUST: &str = "Cargo.toml";
const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const RUST_TASKS: [&str; 4] = [
    "verify-project",
    "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -D clippy::cargo",
    "test -j 4 --no-fail-fast",
    "fmt --check",
];

fn exist(file: &str) -> bool {
    Path::new(file).exists()
}

enum Language {
    Rust,
    Unknown,
}

///
/// # Panics
///
fn check_rust() -> i32 {
    if exist(RUST) {
        for todo in RUST_TASKS {
            let command_part: Vec<&str> = todo.split_whitespace().collect();
            assert!(Command::new("cargo")
                .args(command_part)
                .spawn()
                .unwrap()
                .wait()
                .unwrap()
                .success());
        }
        okay("Your code can be committed");
        return EXIT_SUCCESS;
    }
    okay(format!("No {RUST} has been founded").as_str());
    EXIT_FAILURE
}

fn check(language: &Language) -> i32 {
    match language {
        Rust => check_rust(),
        Unknown => {
            println!("Language not supported");
            EXIT_FAILURE
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

fn all() -> HashMap<String, &'static Language> {
    let mut all: HashMap<String, &Language> = HashMap::new();
    assert!(all.insert(String::from(RUST), &Rust).is_none());
    all
}

fn main() -> ExitCode {
    exit(check(detect()))
}
