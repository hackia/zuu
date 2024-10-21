use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use std::fs::{create_dir_all, File};
use std::io::{stdout, Error, ErrorKind, Stdout};
use std::path::Path;
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[doc = "All checkup to execute for rust"]
const ZUU_RUST_TASK : [&str;9]  =  [
    "verify-project",
    "deny check",
    "check",
    "audit",
    "test",
    "fmt --check",
    "doc --no-deps",
    "outdated",
    "clippy -- -D clippy::cargo -D clippy::complexity -D clippy::style -D clippy::all -D clippy::perf -D clippy::correctness -D clippy::pedantic -D clippy::suspicious"
];
#[doc = "All checkup success messages"]
const ZUU_RUST_TASK_TITLES: [&str; 9] = [
    "Checking the project",
    "Checking project licenses",
    "Checking the build dependencies",
    "Checking security vulnerabilities",
    "Running all tests",
    "Checking code format",
    "Generating documentation",
    "Checking dependencies versions",
    "Checking source code",
];
#[doc = "All checkup success messages"]
const ZUU_RUST_TASK_OK: [&str; 9] = [
    "The project is valid",
    "No dependencies license problem",
    "No packages or dependencies errors",
    "No vulnerabilities has been founded",
    "All test passes",
    "Your code respect standard",
    "Doc generated successfully",
    "All dependencies are up to date",
    "Your code is validated",
];
#[doc = "All checkup failures messages"]
const ZUU_RUST_TASK_KO: [&str; 9] = [
    "The project is not valid",
    "Dependencies license problem has been founded",
    "Packages or dependencies errors has been founded",
    "Security vulnerabilities has been founded",
    "Test have failure",
    "Code don't respect the coding standard",
    "Failed to generate documentation",
    "Dependencies must be updated",
    "Your code contains errors",
];

#[doc = "All checkup failures messages"]
const ZUU_RUST_TASK_OUTPUT_FILES: [&str; 9] = [
    "project",
    "licenses",
    "checks",
    "audit",
    "tests",
    "fmt",
    "documentation",
    "dependencies",
    "lint",
];

#[doc = "The waiting task spinner strings"]
const SPINNERS: [&str; 4] = [". ", "..", ".:", "::"];
#[doc = "All supported languages"]
pub enum Language {
    Rust,
    Go,
    C,
    Cpp,
    D,
    Python,
    Php,
    Java,
    Kotlin,
    Swift,
    Ruby,
    Perl,
    Scala,
    TypeScript,
    Elixir,
    Haskell,
    Clojure,
    Bash,
    ObjectiveC,
    Erlang,
    Lua,
    FSharp, // F#
    R,
    Julia,
    Crystal,
    Groovy,
    Dart,
    Matlab, // MATLAB
    Cobol,
    Fortran,
    Nim,
    Nodejs,
    Vlang, // V language
    OCaml,
    Tcl,
    VHDL,
    Unknown,
}

#[doc = "return the status code of the code checkup"]
#[must_use]
pub const fn zuu_exit(status: &Result<(), Error>) -> ExitCode {
    if status.is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

///
/// # Ok
///
/// Print a success message
///
/// # Panics
///
/// On fail to print the description
///
/// # Errors
///
pub fn ok(output: &mut Stdout, description: &str, x: usize) {
    if let Ok((cols, _rows)) = size() {
        if let Ok(y) = u16::try_from(x) {
            let status: &str = "[ ok ]";
            if let Ok(len) = u16::try_from(status.len()) {
                let status_position: u16 = cols.saturating_sub(len);
                assert!(
                    execute!(
                        output,
                        SetForegroundColor(Color::Green),
                        MoveTo(0, y),
                        Print("*"),
                        MoveTo(2, y),
                        SetForegroundColor(Color::White),
                        Print(description),
                        MoveTo(status_position, 1),
                        SetForegroundColor(Color::Blue),
                        MoveTo(status_position, y),
                        Print("["),
                        SetForegroundColor(Color::Green),
                        Print(" ok "),
                        SetForegroundColor(Color::Blue),
                        Print("]"),
                        SetForegroundColor(Color::Reset),
                    )
                    .is_ok(),
                    "Failed to print success message"
                );
            }
        } else {
            assert!(
                execute!(
                    output,
                    SetForegroundColor(Color::Green),
                    MoveTo(0, 1),
                    Print("*"),
                    MoveTo(2, 1),
                    SetForegroundColor(Color::White),
                    Print(description),
                    SetForegroundColor(Color::Reset),
                )
                .is_ok(),
                "Failed to print success message"
            );
        }
    }
}
///
/// # Ko
///
/// Print an error message
///
/// # Panics
///
/// On fail to print the description
///
pub fn ko(output: &mut Stdout, description: &str, x: usize) {
    if let Ok((cols, _row)) = size() {
        if let Ok(y) = u16::try_from(x) {
            let status: &str = "[ !! ]";
            if let Ok(len) = u16::try_from(status.len()) {
                let status_position: u16 = cols.saturating_sub(len);
                assert!(
                    execute!(
                        output,
                        SetForegroundColor(Color::Red),
                        MoveTo(0, y),
                        Print("*"),
                        MoveTo(2, y),
                        SetForegroundColor(Color::White),
                        Print(description),
                        SetForegroundColor(Color::Blue),
                        MoveTo(status_position, y),
                        Print("["),
                        SetForegroundColor(Color::Red),
                        Print(" !! "),
                        SetForegroundColor(Color::Blue),
                        Print("]"),
                        SetForegroundColor(Color::Reset),
                    )
                    .is_ok(),
                    "Failed to print error message"
                );
            }
        }
    } else {
        assert!(
            execute!(output, Print(description)).is_ok(),
            "Failed to print error message"
        );
    }
}

///
/// # Exec
///
/// Run a check by executing a command
///
/// # Panics
///
/// On cross term failed to print
///
/// # Errors
///
/// On failure
///
pub fn exec(
    output: &mut Stdout,
    description: &'static str,
    cmd: &mut Command,
    f: &'static str,
    x: usize,
) -> std::io::Result<()> {
    let spinner_done = Arc::new(AtomicBool::new(false));
    let spinner_done_clone = Arc::clone(&spinner_done);
    if let Ok((cols, _row)) = size() {
        let status: &str = "   ";
        if let Ok(len) = u16::try_from(status.len()) {
            if let Ok(y) = u16::try_from(x) {
                let status_position: u16 = cols.saturating_sub(len);
                assert!(execute!(
                    output,
                    MoveTo(0, y),
                    SetForegroundColor(Color::Green),
                    Print("*"),
                    MoveTo(2, y),
                    SetForegroundColor(Color::White),
                    Print(description),
                    MoveTo(status_position, y),
                    SetForegroundColor(Color::Green),
                    Print(" "),
                    SetForegroundColor(Color::Reset),
                )
                .is_ok());
                let spinner_thread = thread::spawn(move || {
                    let mut output = stdout();
                    while !spinner_done_clone.load(Ordering::SeqCst) {
                        let status: &str = "[ :: ]";
                        if let Ok(len) = u16::try_from(status.len()) {
                            let status_position: u16 = cols.saturating_sub(len);
                            for spin in SPINNERS {
                                assert!(execute!(
                                    output,
                                    Hide,
                                    SetForegroundColor(Color::Green),
                                    MoveTo(0, y),
                                    Print("*"),
                                    MoveTo(2, y),
                                    SetForegroundColor(Color::White),
                                    Print(description),
                                    MoveTo(status_position, y),
                                    SetForegroundColor(Color::Blue),
                                    Print("["),
                                    SetForegroundColor(Color::Green),
                                    Print(format!(" {spin} ")),
                                    SetForegroundColor(Color::Blue),
                                    Print("]"),
                                    SetForegroundColor(Color::Reset),
                                )
                                .is_ok());
                                sleep(Duration::from_millis(400));
                            }
                        }
                    }
                });
                let output = cmd
                    .stdout(
                        File::create(format!("zuu/stdout/{f}")).expect("failed to create output"),
                    )
                    .stderr(
                        File::create(format!("zuu/stderr/{f}")).expect("failed to create output"),
                    )
                    .spawn()?
                    .wait()?
                    .success();

                spinner_done.store(true, Ordering::SeqCst);
                spinner_thread.join().unwrap();
                assert!(execute!(stdout(), MoveTo(0, y), Clear(ClearType::CurrentLine)).is_ok());
                if output {
                    return Ok(());
                }
            }
        }
    }
    Err(Error::other("a error encountered"))
}
pub struct Zuu {
    language: Language,
}

///
/// # Files output
///
/// Create zuu directories in order to store command output
///
/// # Errors
///
/// On no write rights
///
pub fn create_zuu() -> Result<(), Error> {
    if create_dir_all("zuu").is_ok()
        && create_dir_all("zuu/stdout").is_ok()
        && create_dir_all("zuu/stderr").is_ok()
    {
        Ok(())
    } else {
        Err(Error::other(""))
    }
}

impl Zuu {
    #[must_use]
    pub const fn new(lang: Language) -> Self {
        Self { language: lang }
    }

    #[doc = "Check a cargo project"]
    fn rust(&mut self) -> Result<(), Error> {
        if Path::new("Cargo.toml").is_file() {
            let mut results: Vec<bool> = Vec::new();
            let mut output: Stdout = stdout();
            execute!(&mut output, Clear(ClearType::All)).expect("msg");
            for (i, task) in ZUU_RUST_TASK.iter().enumerate() {
                let title = ZUU_RUST_TASK_TITLES.get(i).unwrap_or(&"NO TITLE");
                let success = ZUU_RUST_TASK_OK.get(i).unwrap_or(&"Success");
                let failure = ZUU_RUST_TASK_KO.get(i).unwrap_or(&"Failure");
                let file = ZUU_RUST_TASK_OUTPUT_FILES.get(i).unwrap_or(&"out");
                if exec(
                    &mut output,
                    title,
                    Command::new("cargo").args(task.split_whitespace()),
                    file,
                    i,
                )
                .is_ok()
                {
                    results.push(true);
                    ok(&mut output, success, i);
                } else {
                    ko(&mut output, failure, i);
                    results.push(false);
                }
            }
            return self.end(&mut output, &results);
        }
        Err(Error::new(ErrorKind::NotFound, "no Cargo.toml"))
    }

    ///
    /// # End
    ///
    /// Close suite test case for a language
    ///
    /// # Panics
    ///
    /// On cross term print failure
    ///
    /// # Errors
    ///
    /// Return error on a checkup failure
    ///
    pub fn end(&mut self, output: &mut Stdout, results: &[bool]) -> Result<(), Error> {
        if results.contains(&false) {
            assert!(execute!(
                output,
                Show,
                SetForegroundColor(Color::Red),
                Print("\n* "),
                SetForegroundColor(Color::White),
                Print("Can't commit check files inside zuu directory for more information\n"),
                SetForegroundColor(Color::Reset)
            )
            .is_ok());
            return Err(Error::other("zuu detect error"));
        }
        assert!(execute!(
            output,
            Show,
            SetForegroundColor(Color::Green),
            Print("\n* "),
            SetForegroundColor(Color::White),
            Print("Source code can be commited\n"),
            SetForegroundColor(Color::Reset)
        )
        .is_ok());
        Ok(())
    }
    fn js(&mut self) -> Result<(), Error> {
        if Path::new("package.json").is_file() {
            let mut results = Vec::<bool>::new();
            let mut output: Stdout = stdout();
            execute!(&mut output, Clear(ClearType::All)).expect("msg");
            if exec(
                &mut output,
                "Auditing source code",
                Command::new("npm").arg("audit"),
                "audit",
                1,
            )
            .is_ok()
            {
                results.push(true);
                ok(&mut output, "No vulnerabilities founded", 1);
            } else {
                ko(&mut output, "Security vulnerabilities detected", 1);
                results.push(false);
            }

            if exec(
                &mut output,
                "Checking dependencies",
                Command::new("npm").arg("outdated"),
                "outdated",
                2,
            )
            .is_ok()
            {
                results.push(true);
                ok(&mut output, "All dependencies are up to date", 2);
            } else {
                ko(&mut output, "Dependencies must be updated", 2);
                results.push(false);
            }
            if exec(
                &mut output,
                "Checking licenses",
                Command::new("npm").arg("run").arg("licenses"),
                "licences",
                3,
            )
            .is_ok()
            {
                results.push(true);
                ok(
                    &mut output,
                    "All dependencies licenses are compatibles to the project",
                    3,
                );
            } else {
                ko(&mut output, "Dependencies licences must be updated", 3);
                results.push(false);
            }
            if exec(
                &mut output,
                "Testing source code",
                Command::new("npm").arg("test"),
                "tests",
                4,
            )
            .is_ok()
            {
                results.push(true);
                ok(&mut output, "All tests passes", 4);
            } else {
                ko(&mut output, "Test have failures", 4);
                results.push(false);
            }

            if exec(
                &mut output,
                "Testing source code format",
                Command::new("npm").arg("run").arg("fmt"),
                "fmt",
                5,
            )
            .is_ok()
            {
                results.push(true);
                ok(&mut output, "Source code respect standard", 5);
            } else {
                ko(&mut output, "Source code must be reformated", 5);
                results.push(false);
            }
            return self.end(&mut output, &results);
        }
        Err(Error::new(ErrorKind::NotFound, "no composer.json"))
    }

    ///
    /// # Run
    ///
    /// Create zuu directories and check source code
    ///
    /// # Panics
    ///
    /// On zuu directories creation structure
    ///
    pub fn check(&mut self) -> ExitCode {
        assert!(
            create_zuu().is_ok(),
            "Failed to create zuu directories structure"
        );
        match self.language {
            Language::Rust => zuu_exit(&self.rust()),
            Language::Nodejs | Language::TypeScript => zuu_exit(&self.js()),
            _ => ExitCode::FAILURE,
        }
    }
}
