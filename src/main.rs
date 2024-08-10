use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{create_dir_all, read_to_string, File};
use std::io::{stdout, ErrorKind, Write};
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::time::Duration;
use std::{io::Error, process::Command};

const ERROR_VERIFY_FILE: &str = "verify";
const ERROR_CHECK_FILE: &str = "check";
const ERROR_DENY_FILE: &str = "deny";
const ERROR_AUDIT_FILE: &str = "audit";
const ERROR_CLIPPY_FILE: &str = "clippy";
const ERROR_TEST_FILE: &str = "test";
const ERROR_FMT_FILE: &str = "format";

const ZUU_TEMPLATE: &str = "{spinner} [{bar:50.white}] {msg}";

const RUST_TASKS: [&str;7] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms -F rust-2021-compatibility -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::todo -F clippy::needless_borrow -F clippy::use_self -F clippy::redundant_clone -F clippy::manual_memcpy -F clippy::manual_assert -F clippy::single_match_else -F clippy::unwrap_used -F clippy::expect_used -F clippy::panic -F clippy::complexity -D clippy::cargo -F keyword_idents -F warnings -F let-underscore",
    "test -j 4 --no-fail-fast",
    "fmt --check"];

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
fn cmd(args: &str, k: &str) -> Result<bool, Error> {
    let x: Vec<&str> = args.split_whitespace().collect();
    if Command::new("cargo")
        .args(x)
        .stdout(File::create(
            format!("zuu{MAIN_SEPARATOR_STR}stdout/{MAIN_SEPARATOR_STR}{k}").as_str(),
        )?)
        .stderr(File::create(
            format!("zuu{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{k}").as_str(),
        )?)
        .current_dir(".")
        .spawn()?
        .wait()?
        .success()
    {
        return Ok(true);
    }
    Err(Error::new(ErrorKind::InvalidData, "error detected"))
}
fn run(k: &Zuu, fileame: &str) -> Result<bool, Error> {
    cmd(prepare_cmd(k).as_str(), fileame)
}
fn prepare_cmd(k: &Zuu) -> String {
    match k {
        Zuu::Verify => RUST_TASKS[0].to_string(),
        Zuu::Check => RUST_TASKS[1].to_string(),
        Zuu::Deny => RUST_TASKS[2].to_string(),
        Zuu::Audit => RUST_TASKS[3].to_string(),
        Zuu::Clippy => RUST_TASKS[4].to_string(),
        Zuu::Test => RUST_TASKS[5].to_string(),
        Zuu::Fmt => RUST_TASKS[6].to_string(),
    }
}
fn zuu_error(k: &Zuu) -> Result<String, Error> {
    let e: String = match k {
        Zuu::Verify => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_VERIFY_FILE}")
                .as_str(),
        )?,
        Zuu::Check => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_CHECK_FILE}")
                .as_str(),
        )?,
        Zuu::Deny => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_DENY_FILE}").as_str(),
        )?,
        Zuu::Audit => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_AUDIT_FILE}")
                .as_str(),
        )?,
        Zuu::Clippy => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_CLIPPY_FILE}")
                .as_str(),
        )?,
        Zuu::Test => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_TEST_FILE}").as_str(),
        )?,
        Zuu::Fmt => read_to_string(
            format!("zuu{MAIN_SEPARATOR_STR}stderr/{MAIN_SEPARATOR_STR}{ERROR_FMT_FILE}").as_str(),
        )?,
    };
    Ok(e)
}
fn zuu() -> Result<(), Error> {
    if Path::new("zuu").exists().eq(&false) {
        create_dir_all("zuu")?;
        create_dir_all(format!("zuu{MAIN_SEPARATOR_STR}stderr"))?;
        create_dir_all(format!("zuu{MAIN_SEPARATOR_STR}stdout"))?;
    }
    if let Ok(style) = ProgressStyle::default_bar().template(ZUU_TEMPLATE) {
        let pb: ProgressBar = ProgressBar::new(7)
            .with_message("Checking source code")
            .with_style(style.progress_chars("== "));

        pb.enable_steady_tick(Duration::from_millis(75));
        for x in &Zuu::all() {
            let filename: String = match x {
                Zuu::Verify => String::from(ERROR_VERIFY_FILE),
                Zuu::Check => String::from(ERROR_CHECK_FILE),
                Zuu::Deny => String::from(ERROR_DENY_FILE),
                Zuu::Audit => String::from(ERROR_AUDIT_FILE),
                Zuu::Clippy => String::from(ERROR_CLIPPY_FILE),
                Zuu::Test => String::from(ERROR_TEST_FILE),
                Zuu::Fmt => String::from(ERROR_FMT_FILE),
            };
            pb.set_message(filename.to_string());
            if run(x, filename.as_str()).is_err() {
                enable_cursor();

                pb.finish_with_message(zuu_error(x)?);
                return Err(Error::new(ErrorKind::InvalidData, zuu_error(x)?));
            }

            pb.inc(1);
        }
        pb.finish_with_message("Code can be commited");
    }
    Ok(())
}

enum Zuu {
    Verify,
    Check,
    Deny,
    Audit,
    Clippy,
    Test,
    Fmt,
}
impl Zuu {
    pub const fn all() -> [Self; 7] {
        [
            Self::Verify,
            Self::Check,
            Self::Deny,
            Self::Audit,
            Self::Clippy,
            Self::Test,
            Self::Fmt,
        ]
    }
}
fn main() {
    disable_cursor();
    clear();
    assert!(zuu().is_ok());
    enable_cursor();
}
