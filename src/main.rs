use crate::helpers::exec;
use crate::helpers::okay;
use crate::Language::{Cmake, Composer, Go, Rust, Unknown};
use std::collections::HashMap;
use std::path::Path;
pub mod helpers;
use std::process::{exit, ExitCode};

enum Language {
    Rust,
    Go,
    Composer,
    Cmake,
    Unknown,
}

///
/// # Panics
///
fn check_cmake() -> i32 {
    assert!(
        exec("sh", &["-c", "cmake . && make || exit 1"]),
        "Failed to build"
    );
    okay("Your code can be committed");
    0
}

///
/// # Panics
///
fn check_rust() -> i32 {
    assert!(
        exec(
            "sh",
            &[
                "-c",
                "cargo build --all-targets --release -j 4 --future-incompat-report && cargo verify-project && cargo bench --no-fail-fast --all-targets --message-format human -j 4 && cargo clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -D clippy::cargo && cargo test --all-targets --all-features --release -j 4 --no-fail-fast && cargo check --all-targets --release --message-format human -j 4 && cargo fmt --check  || exit 1"]),
    "zuu detect failure");

    okay("Your code can be committed");
    0
}

fn check_go() -> i32 {
    assert!(
        exec(
            "sh",
            &["-c", "go mod verify  && go build && go test -v || exit 1"]
        ),
        "zuu detect failure"
    );
    okay("Your code can be committed");
    0
}

fn check(language: &Language) -> i32 {
    match language {
        Rust => check_rust(),
        Go => check_go(),
        Composer => check_composer(),
        Cmake => check_cmake(),
        Unknown => {
            println!("Language not supported");
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

fn check_composer() -> i32 {
    assert!(exec(
        "sh",
        &[
            "-c",
            "composer install && composer audit && composer diagnose || exit 1"
        ]
    ));

    if Path::new("phpunit.xml").exists() && Path::new("vendor").exists() {
        assert!(exec("sh", &["-c", "vendor/bin/phpunit || exit 1"]));
    }

    if Path::new("phpstan.neon").exists() && Path::new("vendor").exists() {
        assert!(exec("sh", &["-c", "vendor/bin/phpstan || exit 1"]));
    }
    okay("Your code can be committed");
    0
}
fn all() -> HashMap<String, &'static Language> {
    let mut all: HashMap<String, &Language> = HashMap::new();
    assert!(all.insert(String::from("Cargo.toml"), &Rust).is_none());
    assert!(all.insert(String::from("go.mod"), &Go).is_none());
    assert!(all
        .insert(String::from("composer.json"), &Composer)
        .is_none());
    assert!(all.insert(String::from("CMakeLists.txt"), &Cmake).is_none());
    all
}

fn main() -> ExitCode {
    exit(check(detect()))
}
