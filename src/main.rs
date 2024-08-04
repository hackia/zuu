use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::{fs::File, io::Error, process::Command};

const RUST_TASKS: [&str;5] = [
    "deny check",
    "audit",
    "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::todo -F clippy::needless_borrow -F clippy::use_self -F clippy::redundant_clone -F clippy::manual_memcpy -F clippy::manual_assert -F clippy::single_match_else -F clippy::unwrap_used -F clippy::expect_used -F clippy::panic -F clippy::complexity -D clippy::cargo     -F keyword_idents -F warnings -F let-underscore",
    "test -j 4 --no-fail-fast",
    "fmt --check"];

fn main() -> Result<(), Error> {
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
            continue;
        }

        eprintln!(
            "{}",
            read_to_string(format!(
                "zuu{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{}",
                p[0]
            ))?
        );
        return Err(Error::last_os_error());
    }
    Ok(())
}
