use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use std::fs::{create_dir_all, File};
use std::io::{stdout, Error, Stdout};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[doc = "All checkup to execute for rust"]
pub const RUST_TASK : [&str;9]  =  [
    "cargo verify-project",
    "cargo deny check",
    "cargo check",
    "cargo audit",
    "cargo test",
    "cargo fmt --check",
    "cargo doc --no-deps",
    "cargo outdated",
    "cargo clippy -- -D clippy::cargo -D clippy::complexity -D clippy::style -D clippy::all -D clippy::perf -D clippy::correctness -D clippy::pedantic -D clippy::suspicious"
];

#[doc = "All checkup to execute for php"]
pub const PHP_TASK: [&str; 9] = [
    "composer validate",
    "composer licenses",
    "composer check-platform-reqs",
    "composer audit",
    "composer run-script test",
    "composer run-script fmt",
    "composer run-script doc",
    "composer outdated",
    "composer run-script lint",
];

#[doc = "All checkup success messages"]
pub const ZUU_TITLES: [&str; 9] = [
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
pub const ZUU_OK: [&str; 9] = [
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
pub const ZUU_KO: [&str; 9] = [
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
pub const ZUU_OUTPUT_FILES: [&str; 9] = [
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
                return if output {
                    Ok(())
                } else {
                    Err(Error::other("Zuu detect error"))
                };
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

    ///
    /// # Rust
    ///
    /// check rust code
    ///
    /// # Errors
    ///
    /// On check failures
    ///
    ///
    pub fn check(&mut self, tasks: &[&str; 9]) -> Result<(), Error> {
        let mut results: Vec<bool> = Vec::new();
        let mut output: Stdout = stdout();
        execute!(&mut output, Clear(ClearType::All)).expect("msg");
        for (i, task) in tasks.iter().enumerate() {
            let title = ZUU_TITLES.get(i).unwrap_or(&"NO TITLE");
            let success = ZUU_OK.get(i).unwrap_or(&"Success");
            let failure = ZUU_KO.get(i).unwrap_or(&"Failure");
            let file = ZUU_OUTPUT_FILES.get(i).unwrap_or(&"out");
            if exec(
                &mut output,
                title,
                Command::new("sh").args(["-c", task]).current_dir("."),
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
        self.end(&mut output, &results)
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

    ///
    /// # Run
    ///
    /// Create zuu directories and check source code
    ///
    /// # Panics
    ///
    /// On zuu directories creation structure
    ///
    pub fn run(&mut self) -> ExitCode {
        assert!(
            create_zuu().is_ok(),
            "Failed to create zuu directories structure"
        );
        match self.language {
            Language::Rust => zuu_exit(&self.check(&RUST_TASK)),
            Language::Php => zuu_exit(&self.check(&PHP_TASK)),
            _ => ExitCode::FAILURE,
        }
    }
}
