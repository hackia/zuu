use crate::output::{ko, ok};
use crate::support::{Language, Support};
use cli_table::{format::Align, format::Justify, Color, Table};
use crossterm::cursor::Show;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use inquire::{Confirm, MultiSelect};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::ExitCode;

#[doc = "All checkup title messages"]
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
    "The project structure is invalid.",
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
#[doc = "The tux configuration"]
pub struct Config {
    pub languages: Vec<String>,
    pub strict: bool,
}

#[doc = "The pro"]
pub const PROJECT_STRUCTURE_VALID: &str = "VALID";
pub const PROJECT_STRUCTURE_NOT_VALID: &str = "NOT VALID";

pub const PROJECT_LICENSE_VALID: &str = "COMPATIBLE";
pub const PROJECT_LICENSE_NOT_VALID: &str = "NOT COMPATIBLE";

pub const BUILD_DEPENDENCIES_VALID: &str = "YES";
pub const BUILD_DEPENDENCIES_NOT_VALID: &str = "NO";

pub const TESTS_RESULT_VALID: &str = "PASS";
pub const TESTS_RESULTS_NOT_VALID: &str = "FAIL";

pub const RESPECT_OF_STANDARD_VALID: &str = "RESPECTED";
pub const RESPECT_OF_STANDARD_NOT_VALID: &str = "NOT RESPECTED";

pub const AUDIT_VALID: &str = "YES";
pub const AUDIT_NOT_VALID: &str = "NO";

pub const CODE_VALID: &str = "APPROVED";
pub const CODE_NOT_VALID: &str = "REJECTED";

pub const OUTDATED_VALID: &str = "UP TO DATE";
pub const OUTDATED_NOT_VALID: &str = "OUTDATED";

pub const DOCUMENTED_VALID: &str = "GENERATED";
pub const DOCUMENTED_NOT_VALID: &str = "NOT GENERATED";

pub const SUCCESS: i32 = 0;
pub const FAILURE: i32 = 1;

#[derive(Table)]
pub struct Report {
    #[table(
        title = "LANGUAGE",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub language: String, // Rust
    #[table(
        title = "PROJECT",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub project_structure: String, // Valid | Not valid
    #[table(title = "LICENSES", justify = "Justify::Left", color = "Color::White")]
    pub licenses: String, // Compatible | Not Compatible
    #[table(
        title = "INSTALLABLE",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub dependencies: String, // YES | NO
    #[table(
        title = "SECURE",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub audit: String, // Vulnerabilities Founded | No Vulnerabilities
    #[table(
        title = "TEST",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub test: String, // PASS | FAIL
    #[table(title = "SECURE", justify = "Justify::Left", color = "Color::White")]
    pub secure: String,
    #[table(
        title = "STANDARD",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub standard: String, // Respected | Not Respected
    #[table(
        title = "DOC",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub documented: String, // Generated | Not Generated
    #[table(
        title = "DEPENDENCIES",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub outdated: String, // Up To Date | Outdated
    #[table(
        title = "SOURCE CODE",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub lint: String, // Approved | Rejected
    #[table(
        title = "EXIT",
        align = "Align::Top",
        justify = "Justify::Left",
        color = "Color::White"
    )]
    pub code: i32, // 1 | 0
}

impl Report {
    #[must_use]
    pub fn new() -> Self {
        Self {
            language: Language::Unknown.to_string(),
            project_structure: PROJECT_STRUCTURE_NOT_VALID.to_uppercase(),
            licenses: PROJECT_LICENSE_NOT_VALID.to_uppercase(),
            dependencies: BUILD_DEPENDENCIES_NOT_VALID.to_uppercase(),
            audit: AUDIT_NOT_VALID.to_uppercase(),
            test: TESTS_RESULTS_NOT_VALID.to_uppercase(),
            secure: AUDIT_NOT_VALID.to_string(),
            standard: RESPECT_OF_STANDARD_NOT_VALID.to_uppercase(),
            documented: DOCUMENTED_NOT_VALID.to_uppercase(),
            outdated: OUTDATED_NOT_VALID.to_uppercase(),
            lint: CODE_NOT_VALID.to_uppercase(),
            code: FAILURE,
        }
    }
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

///
///
/// # Panics
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
    .unwrap_or(Vec::from(["Rust".to_string()]));
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
    assert!(execute!(stdout(), Clear(ClearType::All)).is_ok());
    ko("Failed to generate config : /tux.toml", 0);
    ExitCode::FAILURE
}
