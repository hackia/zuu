#![allow(clippy::multiple_crate_versions)]
use clap::{ArgMatches, Command};
use cli_table::{print_stdout, WithTitle};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType},
};
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::stdout,
    process::{Command as Tux, ExitCode},
};
use zuu::{
    ask::{
        init, Config, Report, AUDIT_NOT_VALID, AUDIT_VALID, BUILD_DEPENDENCIES_NOT_VALID,
        BUILD_DEPENDENCIES_VALID, CODE_NOT_VALID, CODE_VALID, DOCUMENTED_NOT_VALID,
        DOCUMENTED_VALID, FAILURE, OUTDATED_NOT_VALID, OUTDATED_VALID, OUTPUT_FILES,
        PROJECT_LICENSE_NOT_VALID, PROJECT_LICENSE_VALID, PROJECT_STRUCTURE_NOT_VALID,
        PROJECT_STRUCTURE_VALID, RESPECT_OF_STANDARD_NOT_VALID, RESPECT_OF_STANDARD_VALID, SUCCESS,
        TESTS_RESULTS_NOT_VALID, TESTS_RESULT_VALID,
    },
    output::waiting,
    runner::create_zuu,
    support::{Language, Support},
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
    let reports: Vec<Report> = check_source_code();

    assert!(execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0), Show).is_ok());
    assert!(print_stdout(reports.with_title()).is_ok());
    for report in &reports {
        if report.code.eq(&1) {
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
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
        "/",  // Root directory expansion
        "./", // Execution
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
pub fn check_source_code() -> Vec<Report> {
    let config: Config = load_config();
    let mut reports: Vec<Report> = Vec::new();
    let to_check: Vec<String> = config.languages;
    for lang in &Support::new().supported() {
        if to_check.contains(&lang.to_string()) {
            if let Ok(report) = source_code_verify(lang, config.strict) {
                reports.push(report);
            }
        }
    }
    reports
}

fn source_code_verify(l: &Language, strict: bool) -> std::io::Result<Report> {
    assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
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
    assert!(create_dir_all(format!("zuu/{l}")).is_ok());
    assert!(create_dir_all(format!("zuu/{l}/stderr")).is_ok());
    assert!(create_dir_all(format!("zuu/{l}/stdout")).is_ok());
    let mut waiting_line: usize = 0;
    let mut ret: Report = Report::new();
    ret.language = l.to_string();
    for (index, command) in todo.iter().enumerate() {
        waiting_line = index;
        if contains_dangerous_chars(command.1) {
            assert!(waiting(
                (
                    format!(
                        "Stopped bedore task {}: {}/9. Dangerous command founded",
                        command.0,
                        index + 1,
                    ),
                    "Ok let's go".to_string(),
                    "Oops".to_string()
                ),
                Tux::new("sleep").arg("10"),
                waiting_line
            )
            .is_ok());
            assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
            break;
        }
        let data: (String, String, String) = (
            command.0.to_string(), // title
            command.2.to_string(), // success
            command.3.to_string(), // failure
        );
        let report_error: bool = waiting(
            data,
            Tux::new("sh")
                .arg("-c")
                .arg(command.1)
                .stderr(
                    File::create(
                        format!(
                            "zuu/{l}/stderr/{}",
                            OUTPUT_FILES.get(index).unwrap_or(&"default")
                        )
                        .as_str(),
                    )
                    .expect(""),
                )
                .stdout(
                    File::create(
                        format!(
                            "zuu/{l}/stdout/{}",
                            OUTPUT_FILES.get(index).unwrap_or(&"default")
                        )
                        .as_str(),
                    )
                    .expect("msg"),
                ),
            index,
        )
        .is_err();
        results.push(report_error);
        match index {
            0 => {
                ret.project_structure = if report_error {
                    PROJECT_STRUCTURE_NOT_VALID.to_uppercase()
                } else {
                    PROJECT_STRUCTURE_VALID.to_uppercase()
                }
            }
            1 => {
                ret.licenses = if report_error {
                    PROJECT_LICENSE_NOT_VALID.to_uppercase()
                } else {
                    PROJECT_LICENSE_VALID.to_uppercase()
                }
            }
            2 => {
                ret.dependencies = if report_error {
                    BUILD_DEPENDENCIES_NOT_VALID.to_uppercase()
                } else {
                    BUILD_DEPENDENCIES_VALID.to_uppercase()
                }
            }
            3 => {
                ret.audit = if report_error {
                    AUDIT_NOT_VALID.to_uppercase()
                } else {
                    AUDIT_VALID.to_uppercase()
                }
            }
            4 => {
                ret.test = if report_error {
                    TESTS_RESULTS_NOT_VALID.to_uppercase()
                } else {
                    TESTS_RESULT_VALID.to_uppercase()
                }
            }
            5 => {
                ret.standard = if report_error {
                    RESPECT_OF_STANDARD_NOT_VALID.to_uppercase()
                } else {
                    RESPECT_OF_STANDARD_VALID.to_uppercase()
                }
            }
            6 => {
                ret.documented = if report_error {
                    DOCUMENTED_NOT_VALID.to_uppercase()
                } else {
                    DOCUMENTED_VALID.to_uppercase()
                }
            }
            7 => {
                ret.outdated = if report_error {
                    OUTDATED_NOT_VALID.to_uppercase()
                } else {
                    OUTDATED_VALID.to_uppercase()
                }
            }
            8 => {
                ret.lint = if report_error {
                    CODE_NOT_VALID.to_uppercase()
                } else {
                    CODE_VALID.to_uppercase()
                }
            }
            _ => {}
        }
        if strict && report_error.eq(&true) {
            ret.code = FAILURE;
            assert!(waiting(
                (
                    format!("Exiting {} (strict mode): {}/9.", command.0, index + 1),
                    format!("Exiting the {l} test"),
                    format!("Exiting the {l} test"),
                ),
                Tux::new("sleep").arg("10"),
                waiting_line
            )
            .is_ok());
            assert!(execute!(stdout(), Clear(ClearType::All), Hide).is_ok());
            break;
        }
    }
    if results.contains(&true) {
        ret.code = FAILURE;
        assert!(waiting(
            (
                format!("Exit code failure for {l} test"),
                format!("Exiting the {l} test"),
                format!("Exiting the {l} test"),
            ),
            Tux::new("sleep").arg("10"),
            waiting_line + 1
        )
        .is_ok());
        assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
        Ok(ret)
    } else {
        ret.code = SUCCESS;
        assert!(waiting(
            (
                format!("Exit code success for {l} test"),
                format!("Exiting the {l} test"),
                format!("Exiting the {l} test"),
            ),
            Tux::new("sleep").arg("10"),
            waiting_line + 1
        )
        .is_ok());
        assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
        Ok(ret)
    }
}
