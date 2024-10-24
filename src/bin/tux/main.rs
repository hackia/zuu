#![allow(clippy::multiple_crate_versions)]
use clap::{ArgMatches, Command};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::Clear,
};
use std::{
    fs::read_to_string,
    io::{stdout, Error},
    process::{Command as Tux, ExitCode},
};

use zuu::{
    ask::{init, Config, OUTPUT_FILES, ZUU_KO, ZUU_OK, ZUU_TITLES},
    output::{exec, ko},
    runner::create_zuu,
    support::Language,
    D_TASK, GO_TASK, JS_TASK, PHP_TASK, PYTHON_TASK, RUST_TASK,
};

#[doc = "command line options"]
fn tux() -> ArgMatches {
    Command::new("tux")
        .bin_name("tux")
        .author("Willy Micieli")
        .color(clap::ColorChoice::Always)
        .long_about(
            "Tux is a command-line tool for checking the source code of your project. \
It runs a series of validation tasks, including format checks, dependency audits, and security scans, \
to ensure your code meets quality standards. You can choose to run all tasks in sequence or stop on the first failure \
by using the strict mode. Tux also helps you set up source tracking by creating a tux.toml configuration file."
        )
        .about(
            "Tux checks your project’s source code quality by running various validation tasks."
        )
        .subcommand(
            Command::new("strict")
                .about("Exit on the first failure during validation")
                .subcommand_required(false),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize the source tracking by creating a tux.toml configuration file")
                .subcommand_required(false),
        )
        .get_matches()
}

#[must_use]
pub fn main() -> ExitCode {
    assert!(execute!(
        stdout(),
        Clear(crossterm::terminal::ClearType::All),
        Hide,
        MoveTo(0, 0)
    )
    .is_ok());
    assert!(create_zuu().is_ok());
    let app: ArgMatches = tux();
    if app.subcommand_matches("init").is_some() {
        return init();
    }
    assert!(execute!(stdout(), Show).is_ok());
    if check_source_code().contains(&false) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

#[doc = "load user configuration"]
fn load_config() -> Config {
    if let Ok(config) = read_to_string("tux.toml") {
        if let Ok(tux) = toml::from_str::<Config>(&config) {
            return tux;
        }
    }
    assert!(execute!(stdout(), Show).is_ok());
    panic!("Failed to load config => run tux init");
}

pub fn contains_dangerous_chars(command: &str) -> bool {
    let dangerous_chars = [
        ";",  // Command separator
        "&",  // Background execution
        "|",  // Pipe to chain commands
        "`",  // Command substitution
        "$(", // Start of command substitution (alternative to `)
        "${", // Variable substitution (can be used for injection)
        ">",  // Output redirection
        "<",  // Input redirection
        ">>", // Append output redirection
        "<<", // Multiple input redirection
        "*",  // Wildcard (can be exploited)
        "?",  // Wildcard (can be exploited)
        "[",  // Start of a regex or character class (can be used in commands)
        "]",  // End of a regex or character class
        "~",  // Home directory expansion
        "^",  // Substitution in some shells
        "!",  // Command history in some shells
        "\\", // Escape character
        "||", // Logical OR
        "&&", // Logical AND
        "(",  // Open parentheses (used for subshells or grouping)
        ")",  // Close parentheses
        "#",  // Comment in many shells (can hide parts of commands)
    ];
    dangerous_chars.iter().any(|&c| command.contains(c))
}
///
/// # Panics
///
/// On failed parse config or crossterm faillure
///
#[must_use]
pub fn check_source_code() -> Vec<bool> {
    let config: Config = load_config();
    let mut data: Vec<bool> = Vec::new();
    for x in &config.languages {
        match x.to_lowercase().as_str() {
            "rust" => {
                data.push(
                    source_code_verify(&Language::Rust, config.strict, &config.output.style)
                        .is_ok(),
                );
            }
            "js" => data.push(
                source_code_verify(&Language::JavaScript, config.strict, &config.output.style)
                    .is_ok(),
            ),
            "php" => {
                data.push(
                    source_code_verify(&Language::Php, config.strict, &config.output.style).is_ok(),
                );
            }
            "python" => data.push(
                source_code_verify(&Language::Python, config.strict, &config.output.style).is_ok(),
            ),
            "go" => {
                data.push(
                    source_code_verify(&Language::Go, config.strict, &config.output.style).is_ok(),
                );
            }
            "d" => data.push(
                source_code_verify(&Language::D, config.strict, &config.output.style).is_ok(),
            ),
            _ => data.push(
                source_code_verify(&Language::Unknown, config.strict, &config.output.style)
                    .is_err(),
            ),
        }
    }
    data
}

fn source_code_verify(l: &Language, strict: bool, style: &str) -> Result<(), Error> {
    let mut results: Vec<bool> = Vec::new();
    let todo = match l {
        Language::Rust => RUST_TASK,
        Language::JavaScript => JS_TASK,
        Language::TypeScript => JS_TASK,
        Language::Go => GO_TASK,
        Language::D => D_TASK,
        Language::Python => PYTHON_TASK,
        Language::Php => PHP_TASK,
        Language::Unknown => RUST_TASK,
    };
    for (index, command) in todo.iter().enumerate() {
        let title = ZUU_TITLES.get(index).unwrap_or(&"checking");
        let filename = OUTPUT_FILES.get(index).unwrap_or(&"latest");
        let success_message = ZUU_OK.get(index).unwrap_or(&"success");
        let failure_message = ZUU_KO.get(index).unwrap_or(&"failure");
        if contains_dangerous_chars(command) {
            ko(
                &mut stdout(),
                style,
                format!(
                    "Stopped bedore task {title}: {}/9. Dangerous command founded",
                    index + 1
                )
                .as_str(),
                index,
            );
            break;
        }
        let data = (
            title.to_string(),
            style.to_string(),
            success_message.to_string(),
            failure_message.to_string(),
            filename.to_string(),
        );
        results.push(
            exec(
                &mut stdout(),
                data,
                Tux::new("sh").arg("-c").arg(command),
                index,
            )
            .is_ok(),
        );
        if strict && results.contains(&false) {
            ko(
                &mut stdout(),
                style,
                format!("Stopped after task {title}: {}/9.", index + 1).as_str(),
                index,
            );
            assert!(execute!(stdout(), Show).is_ok());
            break;
        }
    }
    assert!(execute!(stdout(), Show).is_ok());
    if results.contains(&false) {
        return Err(Error::other("Err"));
    }
    Ok(())
}
