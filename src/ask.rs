use crate::support::Support;
use crossterm::cursor::Show;
use crossterm::execute;
use inquire::{Confirm, MultiSelect, Select};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;
use std::process::ExitCode;

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
pub struct OutputConfig {
    pub style: String,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationConfig {
    pub summary: String,
    pub body: String,
    pub urgency: String,
    pub expire: u32,
}

#[derive(Serialize, Deserialize)]
pub struct OnConfig {
    pub failure: NotificationConfig,
    pub success: NotificationConfig,
}

#[derive(Deserialize, Serialize)]
pub struct BadgeConfig {
    pub generate: bool,
    pub output_dir: String,
    pub output_template: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub languages: Vec<String>,
    pub notify: bool,
    pub strict: bool,
    pub output: OutputConfig,
    pub badges: BadgeConfig,
    pub on: OnConfig,
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
        "Select the languages used in your project:",
        Support::new().all(),
    )
    .prompt()
    .expect("Failed to get language selection");

    let notify = Confirm::new("Do you want to enable notifications ?")
        .with_default(true)
        .prompt()
        .unwrap_or_default();
    let output_style = Select::new(
        "Select the output style:",
        vec!["openrc", "bar", "systemd", "default"],
    )
    .prompt()
    .unwrap_or("openrc");
    let generate_badges = Confirm::new("Do you want to generate badges ?")
        .with_default(true)
        .prompt()
        .unwrap_or_default();

    let config_content: String = format!(
        r#"
languages = [{}]
notify = {notify}
strict = false

[output]
style = "{output_style}"

[badges]
generate = {generate_badges}
output_dir = "badges"
output_template = "{{output_dir}}/{{language}}/{{task}}.svg"

[on]
failure = {{ summary = "Task failed", body = "", urgency = "critical", expire = 5000 }}
success = {{ summary = "All tasks passed", body = "", urgency = "normal", expire = 5000 }}
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
        println!("tux.toml has been generated successfully.");
        return ExitCode::SUCCESS;
    }
    ExitCode::FAILURE
}
