use crate::helpers::exec;
use crate::helpers::okay;
use crate::helpers::project;
use crate::Language::{Cmake, Composer, Go, Make, Rust, Unknown};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::read_to_string;
use std::path::Path;
use std::process::exit;
pub mod helpers;

enum Language {
    Rust,
    Go,
    Make,
    Composer,
    Cmake,
    Unknown,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(File::open(filename).expect("failed to found"))
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn check_make() -> i32 {
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
        assert!(exec("sh", &["-c", format!("make {x}").as_str()]));
    }
    0
}

fn check_cmake() -> i32 {
    assert!(
        exec(
            "sh",
            &["-c", "cmake . > zuu/stdout/cmake 2> zuu/stderr/cmake"]
        ),
        "Failed to build Makefile"
    );
    assert!(
        exec("sh", &["-c", "make > zuu/stdout/make 2> zuu/stderr/make"]),
        "Failed to build"
    );
    okay("Your code can be committed");
    0
}

fn check_rust() -> i32 {
    assert!(
        exec(
            "sh",
            &[
                "-c",
                "cargo verify-project > zuu/stdout/project 2> zuu/stderr/project"
            ]
        ),
        "Your project is not valid"
    );
    assert!(exec("sh",&["-c","cargo bench --no-fail-fast --all-targets --message-format human -j 4 > zuu/stdout/bench 2> zuu/stderr/bench"]),"Bench detect errors");
    assert!(exec("sh",&["-c","cargo build --all-targets --release -j 4 --future-incompat-report > zuu/stdout/build 2> zuu/stderr/build"]));
    assert!(exec(
        "sh",
        &["-c", "cargo auditable build --release > zuu/stdout/audit-build 2> zuu/stderr/audit-build"]
    ),"");
    assert!(
        exec(
            "sh",
            &[
                "-c",
                format!(
                    "cargo audit bin target/release/{} > zuu/stdout/audit 2> zuu/stderr/audit",
                    project()
                )
                .as_str()
            ]
        ),
        "Audit found vunerabiliies"
    );

    assert!(exec("sh",&["-c","cargo clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -D clippy::cargo > zuu/stdout/clippy 2> zuu/stderr/clippy"]),"Clippy detect errors");
    assert!(exec("sh",&["-c","cargo test --all-targets --all-features --release -j 4 --no-fail-fast > zuu/stdout/test 2> zuu/stderr/test"]),"Test detect failures");
    assert!(exec("sh",&["-c","cargo check --all-targets --release --message-format human -j 4 > zuu/stdout/check 2> zuu/stderr/check"]),"Check detect error");
    assert!(
        exec(
            "sh",
            &[
                "-c",
                "cargo fmt --check > zuu/stdout/format 2> zuu/stderr/format"
            ]
        ),
        "Your code is not formated correctless"
    );
    assert!(
        exec(
            "sh",
            &[
                "-c",
                "cargo deny check > zuu/stdout/deny 2> zuu/stderr/deny"
            ]
        ),
        "Deny detect errors"
    );
    okay("Your code can be committed");
    0
}

fn check_go() -> i32 {
    assert!(exec(
        "sh",
        &[
            "-c",
            "go mod verify > zuu/stdout/project 2> zuu/stderr/project"
        ]
    ));
    assert!(exec(
        "sh",
        &["-c", "go build > zuu/stdout/build 2> zuu/stderr/build"]
    ));
    assert!(exec(
        "sh",
        &["-c", "go test -v > zuu/stdout/tests 2> zuu/stderr/tests"]
    ));
    okay("Your code can be committed");
    0
}

fn check(language: &Language) -> i32 {
    match language {
        Rust => check_rust(),
        Go => check_go(),
        Make => check_make(),
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
            "composer install > zuu/stdout/install 2> zuu/stderr/install"
        ]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            "composer audit > zuu/stdout/audit 2> zuu/stderr/audit"
        ]
    ));
    assert!(exec(
        "sh",
        &[
            "-c",
            "composer diagnose > zuu/stdout/diagnose 2> zuu/stderr/diagnose"
        ]
    ));
    if Path::new("phpunit.xml").exists() && Path::new("vendor").exists() {
        assert!(exec(
            "sh",
            &[
                "-c",
                "vendor/bin/phpunit > zuu/stdout/tests 2> zuu/stderr/tests"
            ]
        ));
    }

    if Path::new("phpstan.neon").exists() && Path::new("vendor").exists() {
        assert!(exec(
            "sh",
            &[
                "-c",
                "vendor/bin/phpstan > zuu/stdout/analyser 2> zuu/stderr/analyser"
            ]
        ));
    }
    okay("Your code can be committed");
    0
}
fn all() -> HashMap<String, &'static Language> {
    let mut all: HashMap<String, &Language> = HashMap::new();
    assert!(all.insert(String::from("Cargo.toml"), &Rust).is_none());
    assert!(all.insert(String::from("go.mod"), &Go).is_none());
    assert!(all.insert(String::from("Makefile"), &Make).is_none());
    assert!(all
        .insert(String::from("composer.json"), &Composer)
        .is_none());
    assert!(all.insert(String::from("CMakeLists.txt"), &Cmake).is_none());
    all
}

fn main() {
    if !Path::new("zuu").exists() {
        fs::create_dir("zuu").expect("Failed to create zuu");
        fs::create_dir("zuu/stdout").expect("Failed to create stdout");
        fs::create_dir("zuu/stderr").expect("Failet to create stderr");
    }
    exit(check(detect()));
}
