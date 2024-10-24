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
    ask::{init, Config, OUTPUT_FILES},
    output::{exec, ko},
    runner::create_zuu,
    support::Language,
    BASH_TASK, CPP_TASK, CRYSTAL_TASK, C_TASK, DART_TASK, D_TASK, ELIXIR_TASK, FISH_TASK,
    FSHARP_TASK, GO_TASK, HASKELL_TASK, JAVA_TASK, KOTLIN_TASK, LUA_TASK, NIM_TASK, NODEJS_TASK,
    OBJC_TASK, PERL_TASK, PHP_TASK, PYTHON_TASK, RUBY_TASK, RUST_TASK, R_TASK, SCALA_TASK,
    SWIFT_TASK, ZSH_TASK,
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
pub fn load_config() -> Config {
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
        Language::Go => GO_TASK,
        Language::D => D_TASK,
        Language::Python => PYTHON_TASK,
        Language::Php => PHP_TASK,
        Language::Unknown => RUST_TASK,
        Language::JavaScript => NODEJS_TASK,
        Language::TypeScript => NODEJS_TASK,
        Language::Java => JAVA_TASK,
        Language::Kotlin => KOTLIN_TASK,
        Language::Swift => SWIFT_TASK,
        Language::Scala => SCALA_TASK,
        Language::Ruby => RUBY_TASK,
        Language::Perl => PERL_TASK,
        Language::R => R_TASK,
        Language::Haskell => HASKELL_TASK,
        Language::Lua => LUA_TASK,
        Language::ObjectiveC => OBJC_TASK,
        Language::C => C_TASK,
        Language::Cpp => CPP_TASK,
        Language::Nim => NIM_TASK,
        Language::Crystal => CRYSTAL_TASK,
        Language::FSharp => FSHARP_TASK,
        Language::Dart => DART_TASK,
        Language::Elixir => ELIXIR_TASK,
        Language::Bash => BASH_TASK,
        Language::Zsh => ZSH_TASK,
        Language::Fish => FISH_TASK,
    };
    for (index, command) in todo.iter().enumerate() {
        if contains_dangerous_chars(command.1) {
            ko(
                &mut stdout(),
                style,
                format!(
                    "Stopped bedore task {}: {}/9. Dangerous command founded",
                    command.0,
                    index + 1,
                )
                .as_str(),
                index,
            );
            break;
        }
        let data: (String, String, String, String, String) = (
            command.0.to_string(),
            command.1.to_string(),
            command.2.to_string(),
            command.3.to_string(),
            OUTPUT_FILES.get(index).unwrap_or(&"default").to_string(),
        );
        results.push(
            exec(
                &mut stdout(),
                data,
                Tux::new("sh").arg("-c").arg(command.1),
                index,
            )
            .is_ok(),
        );
        if strict && results.contains(&false) {
            ko(
                &mut stdout(),
                style,
                format!("Stopped after task {}: {}/9.", command.0, index + 1).as_str(),
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
