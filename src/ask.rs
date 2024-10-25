use crate::output::{ko, ok};
use crate::support::Support;
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use inquire::{Confirm, MultiSelect};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::ExitCode;
use tabled::Tabled;

#[doc = "All checkup success messages"]
pub const ZUU_TITLES: [&str; 9] = [
    "Validating the project structure",
    "Verifying project licenses",
    "Checking build dependencies",
    "Scanning for security vulnerabilities",
    "Running all tests",
    "Validating code formatting",
    "Generating project documentation",
    "Checking for outdated dependencies",
    "Linting the source code",
];

#[doc = "All check output messages"]
pub const OUTPUT_FILES: [&str; 9] = [
    "project_validation.txt",
    "license_check.txt",
    "dependency_checks.txt",
    "security_audit.txt",
    "test_results.txt",
    "formatting_check.txt",
    "documentation_generation.txt",
    "dependency_updates.txt",
    "code_linting.txt",
];

#[doc = "All checkup success messages"]
pub const ZUU_OK: [&str; 9] = [
    "The project is valid.",
    "No license issues found in dependencies.",
    "No errors found in packages or dependencies.",
    "No security vulnerabilities detected.",
    "All tests passed.",
    "The code meets the formatting standards.",
    "Documentation generated successfully.",
    "All dependencies are up to date.",
    "The code is validated.",
];

#[doc = "All checkup failure messages"]
pub const ZUU_KO: [&str; 9] = [
    "The project is invalid.",
    "License issues detected in dependencies.",
    "Errors found in packages or dependencies.",
    "Security vulnerabilities detected.",
    "Tests failed.",
    "Code does not conform to the formatting standards.",
    "Failed to generate documentation.",
    "Dependencies are outdated and need updating.",
    "The code contains errors.",
];

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub languages: Vec<String>,
    pub strict: bool,
}
#[derive(Tabled)]
pub struct Report {
    pub language: String,
    pub validated: bool,
    pub licenses: bool,
    pub packages: bool,
    pub audit: bool,
    pub test: bool,
    pub secure: bool,
    pub standard: bool,
    pub documented: bool,
    pub outdated: bool,
    pub lint: bool,
    pub code: i32,
}

impl Report {
    pub fn new() -> Self {
        Self {
            language: String::new(),
            validated: false,
            licenses: false,
            packages: false,
            audit: false,
            test: false,
            secure: false,
            standard: false,
            documented: false,
            outdated: false,
            lint: false,
            code: 1,
        }
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}
#[must_use]
pub fn init() -> ExitCode {
    let tux = Path::new("tux.toml");
    if tux.exists()
        && Confirm::new("tux already exist, override ?")
            .with_default(false)
            .prompt()
            .unwrap_or_default()
            .eq(&false)
    {
        return ExitCode::SUCCESS;
    }

    let languages: Vec<String> = MultiSelect::new(
        "Select the languages used in your project :",
        Support::new().all(),
    )
    .prompt()
    .expect("Failed to get language selection");
    let strict = Confirm::new("Do you want to stop the script on the first failure ?")
        .with_default(false)
        .prompt()
        .unwrap_or_default();

    let config_content: String = format!(
        r#"
languages = [{}]
strict = {strict}
"#,
        languages
            .iter()
            .map(|s| format!(r#""{s}""#))
            .collect::<Vec<_>>()
            .join(", "),
    );
    assert!(execute!(stdout(), Show).is_ok());
    if let Ok(mut conf) = File::create(tux) {
        assert!(conf.write_all(config_content.as_bytes()).is_ok());
        assert!(conf.sync_all().is_ok());
        assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
        ok("The config has been generated successfully at tux.toml", 0);
        return ExitCode::SUCCESS;
    }
    ko("Failed to generate config : ./tux.toml", 3);
    ExitCode::FAILURE
}
