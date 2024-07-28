use indicatif::{ProgressBar, ProgressStyle};
use std::env::current_dir;
use std::fs::{read_to_string, File};
use std::path::MAIN_SEPARATOR_STR;
use std::{
    fs::create_dir_all,
    path::Path,
    process::{exit, Command, ExitCode},
    thread::sleep,
    time::Duration,
};
const ZUU_OUT_DIR: &str = "zuu";

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const RUST_TASKS: [&str; 4] = [
    "verify-project",
    "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -D clippy::cargo",
    "test -j 4 --no-fail-fast -- --show-output",
    "fmt --check"
];

const RUST_TASKS_FILENAME: [&str; 4] = ["project.ji", "clippy.ji", "test.ji", "fmt.ji"];

const RUST_TASKS_TITLE_MESSAGE: [&str; 4] = [
    "Check if the project is correctly configured",
    "Check if the source code is correct",
    "Run test",
    "Check code format",
];
const RUST_TASKS_SUCCESS_MESSAGE: [&str; 4] = [
    "The project is correctly configured",
    "Your code is optimized",
    "Test passes checkup",
    "Your code respect correctly formatted",
];

const RUST_TASKS_FAILURES_MESSAGE: [&str; 4] = [
    "The project is not correctly configured",
    "Your code must be optimized",
    "Test no passes checkup",
    "Your code is not correctly formatted",
];

///
/// # Panics
///
fn check(t: &str, x: usize) -> bool {
    let command_part: Vec<&str> = t.split_whitespace().collect();
    Command::new("cargo")
        .args(command_part)
        .stderr(
            File::create(error(RUST_TASKS_FILENAME[x]).as_str())
                .expect("failed to create output filename"),
        )
        .stdout(
            File::create(success(RUST_TASKS_FILENAME[x]).as_str())
                .expect("failed to create output filename"),
        )
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
}
fn error(x: &str) -> String {
    format!("{ZUU_OUT_DIR}{MAIN_SEPARATOR_STR}stderr{MAIN_SEPARATOR_STR}{x}")
}
fn success(x: &str) -> String {
    format!("{ZUU_OUT_DIR}{MAIN_SEPARATOR_STR}stdout{MAIN_SEPARATOR_STR}{x}")
}

///
/// # Panics
///
fn main() -> ExitCode {
    let binding = current_dir().expect("failed to get current dir");
    let project: Option<&str> = binding
        .to_str()
        .expect("msg")
        .split(MAIN_SEPARATOR_STR)
        .last();

    if Path::new(ZUU_OUT_DIR).is_dir().eq(&false) {
        assert!(create_dir_all(format!("{ZUU_OUT_DIR}{MAIN_SEPARATOR_STR}stderr")).is_ok());
        assert!(create_dir_all(format!("{ZUU_OUT_DIR}{MAIN_SEPARATOR_STR}stdout")).is_ok());
    }

    let pb = ProgressBar::new(5)
        .with_message("Check if the code can be commited")
        .with_style(
            ProgressStyle::default_bar()
                .template("[{bar:50.white}] {msg}")
                .expect("")
                .progress_chars("= "),
        );
    for x in 0..4 {
        pb.set_message(RUST_TASKS_TITLE_MESSAGE[x]);
        sleep(Duration::from_millis(500));
        if check(RUST_TASKS[x], x).eq(&false) {
            pb.finish_with_message(RUST_TASKS_FAILURES_MESSAGE[x]);
            println!(
                "\n\nstderr :\n\n{}\nstdout :\n\n{}\nBug report  : <https://github.com/otechdo/zuu/issues>\n\nSource code : <https://github.com/otechdo/zuu>\n\n",
                read_to_string(error(RUST_TASKS_FILENAME[x]).as_str())
                    .expect("failed to get last error"),
                read_to_string(success(RUST_TASKS_FILENAME[x]).as_str())
                    .expect("failed to get last error")
            );
            exit(EXIT_FAILURE);
        } else {
            pb.set_message(RUST_TASKS_SUCCESS_MESSAGE[x]);
        }
        pb.inc(1);
        sleep(Duration::from_millis(500));
    }
    pb.finish_with_message(format!(
        "Congratulations {} your {} project can be commited",
        std::env::var("USER").expect("no USER variable founded"),
        project.expect("failed to get the project name")
    ));
    exit(EXIT_SUCCESS);
}
