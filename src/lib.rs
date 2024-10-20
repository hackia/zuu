use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{size, Clear, ClearType};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{stdout, Error, Stdout};
use std::process::{Command, ExitCode};

pub const FORMAT_ERR: &str = "Source code is not formatted correctly. Please run the formatter.";
pub const AUDIT_ERR: &str = "Security vulnerabilities detected in the code";
pub const TEST_ERR: &str = "Some tests did not pass. Please review the test results.";
pub const LINT_ERR: &str = "Your code does not meet style requirements.";
pub const LICENSE_ERR: &str = "Some dependencies may have incompatible licenses.";

pub const TARGET_FMT: &str = "zuu-fmt";
pub const TARGET_AUDIT: &str = "zuu-audit";
pub const TARGET_TEST: &str = "zuu-test";
pub const TARGET_LICENSE: &str = "zuu-license";
pub const TARGET_LINT: &str = "zuu-lint";

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
#[derive(PartialEq, Eq, Hash)]
pub enum Checked {
    Fmt,
    Audit,
    Test,
    License,
    Lint,
}

pub fn zuu_exit(status: &Result<(), Error>) -> ExitCode {
    if status.is_err() {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
pub fn ok(output: &mut Stdout, description: &str, x: u16) -> std::io::Result<()> {
    let (cols, _rows) = size().expect("failed to get terminal size");

    let status: &str = "[ ok ]";

    let status_len: u16 = status.len() as u16;
    let status_position: u16 = cols.saturating_sub(status_len);

    execute!(
        output,
        MoveTo(0, 0),
        SetForegroundColor(Color::Green),
        MoveTo(0, x),
        Print("*"),
        MoveTo(2, x),
        SetForegroundColor(Color::White),
        Print(description),
        MoveTo(status_position, 1),
        SetForegroundColor(Color::Blue),
        MoveTo(status_position, x),
        Print("["),
        SetForegroundColor(Color::Green),
        Print(" ok "),
        SetForegroundColor(Color::Blue),
        Print("]"),
        SetForegroundColor(Color::Reset),
    )
}

pub fn ko(output: &mut Stdout, description: &str, x: u16) -> std::io::Result<()> {
    let (cols, _rows) = size().expect("failed to get terminal size");

    let status: &str = "[ ok ]";

    let status_len: u16 = status.len() as u16;
    let status_position: u16 = cols.saturating_sub(status_len);

    execute!(
        output,
        MoveTo(0, 0),
        SetForegroundColor(Color::Red),
        MoveTo(0, x),
        Print("*"),
        MoveTo(2, x),
        SetForegroundColor(Color::White),
        Print(description),
        MoveTo(status_position, 1),
        SetForegroundColor(Color::Blue),
        MoveTo(status_position, x),
        Print("["),
        SetForegroundColor(Color::Red),
        Print(" !! "),
        SetForegroundColor(Color::Blue),
        Print("]"),
        SetForegroundColor(Color::Reset),
    )
}
fn check(x: &HashMap<Checked, bool>) -> Result<(), Error> {
    let mut output: Stdout = stdout();
    for (i, v) in x {
        match i {
            Checked::Fmt => {
                if v.eq(&true) {
                    assert!(ok(
                        &mut output,
                        "The source code format respect the standard",
                        0
                    )
                    .is_ok());
                } else {
                }
            }
            Checked::Audit => {
                if v.eq(&true) {
                    assert!(ok(&mut output, "No vulnerabilities has been founded", 1).is_ok());
                } else {
                }
            }
            Checked::Test => {
                if v.eq(&true) {
                    assert!(ok(&mut output, "All tests passes", 1).is_ok());
                } else {
                }
            }
            Checked::License => {
                if v.eq(&true) {
                    assert!(ok(&mut output, "No licences problem has bee founded", 1).is_ok());
                } else {
                }
            }
            Checked::Lint => {
                if v.eq(&true) {
                    assert!(ok(&mut output, "No problem detected", 1).is_ok());
                } else {
                }
            }
        }
    }
    Err(Error::other("Zuu has detected errors"))
}
pub struct Zuu {
    checked: HashMap<Checked, bool>,
    language: Language,
}
impl Zuu {
    pub fn new(lang: Language) -> Self {
        create_dir_all("zuu").expect("msg");
        create_dir_all("zuu/stderr").expect("msg");
        create_dir_all("zuu/stdout").expect("msg");
        Self {
            checked: HashMap::new(),
            language: lang,
        }
    }

    fn rust(&mut self) -> Result<(), Error> {
        let mut results: (bool, bool, bool, bool, bool) = (false, false, false, false, false);
        let mut output: Stdout = stdout();
        execute!(&mut output, Clear(ClearType::All)).expect("msg");
        if Command::new("cargo")
            .arg("deny")
            .arg("check")
            .stderr(File::create("zuu/stderr/license")?)
            .stdout(File::create("zuu/stdout/license")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.0 = true;
            assert!(ok(&mut output, "No license problem founded", 1).is_ok());
        } else {
            assert!(ko(&mut output, LICENSE_ERR, 1).is_ok());
            results.0 = false;
        }
        if Command::new("cargo")
            .arg("audit")
            .stderr(File::create("zuu/stderr/audit")?)
            .stdout(File::create("zuu/stdout/audit")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.1 = true;
            assert!(ok(&mut output, "No vulnerabilities founded", 2).is_ok());
        } else {
            results.1 = false;
            assert!(ko(&mut output, AUDIT_ERR, 2).is_ok());
        }
        if Command::new("cargo")
            .arg("clippy")
            .stderr(File::create("zuu/stderr/lint")?)
            .stdout(File::create("zuu/stdout/lint")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.2 = true;
            assert!(ok(&mut output, "No lint errors founded", 3).is_ok());
        } else {
            results.2 = false;
            assert!(ko(&mut output, LINT_ERR, 3).is_ok());
        }

        if Command::new("cargo")
            .arg("test")
            .arg("--no-fail-fast")
            .stderr(File::create("zuu/stderr/tests")?)
            .stdout(File::create("zuu/stdout/tests")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.3 = true;
            assert!(ok(&mut output, "All tests passes", 4).is_ok());
        } else {
            results.3 = false;
            assert!(ko(&mut output, TEST_ERR, 4).is_ok());
        }
        if Command::new("cargo")
            .arg("fmt")
            .arg("--check")
            .arg("--all")
            .stderr(File::create("zuu/stderr/fmt")?)
            .stdout(File::create("zuu/stdout/fmt")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.4 = true;
            assert!(ok(&mut output, "Source code format respect stantard", 5).is_ok());
        } else {
            results.4 = false;
            assert!(ko(&mut output, FORMAT_ERR, 5).is_ok());
        }
        assert!(execute!(&mut output, Print("\n\n")).is_ok());
        if results.0 && results.1 && results.2 && results.3 && results.4 {
            return Ok(());
        }
        Err(Error::other("zuu detect error"))
    }

    fn php(&mut self) -> Result<(), Error> {
        let mut results: (bool, bool, bool, bool, bool, bool) =
            (false, false, false, false, false, false);
        let mut output: Stdout = stdout();
        execute!(&mut output, Clear(ClearType::All)).expect("msg");
        if Command::new("composer")
            .arg("validate")
            .arg("--strict")
            .stderr(File::create("zuu/stderr/validate")?)
            .stdout(File::create("zuu/stdout/validate")?)
            .current_dir(".")
            .spawn()
            .expect("composer")
            .wait()
            .expect("wait")
            .success()
        {
            results.0 = true;
            assert!(ok(&mut output, "No composer problem founded", 1).is_ok());
        } else {
            assert!(ko(&mut output, "Composer validate detect problem", 1).is_ok());
            results.0 = false;
        }
        if Command::new("composer")
            .arg("diagnose")
            .stderr(File::create("zuu/stderr/diagnose")?)
            .stdout(File::create("zuu/stdout/diagnose")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.1 = true;
            assert!(ok(&mut output, "Diagnose no detect problem", 2).is_ok());
        } else {
            results.1 = false;
            assert!(ko(&mut output, "Diagnose detect problem", 2).is_ok());
        }
        if Command::new("composer")
            .arg("audit")
            .stderr(File::create("zuu/stderr/audit")?)
            .stdout(File::create("zuu/stdout/audit")?)
            .current_dir(".")
            .spawn()
            .expect("cargo")
            .wait()
            .expect("wait")
            .success()
        {
            results.2 = true;
            assert!(ok(&mut output, "No audit errors founded", 3).is_ok());
        } else {
            results.2 = false;
            assert!(ko(&mut output, "Audit errors has been founded", 3).is_ok());
        }

        if Command::new("composer")
            .arg("test")
            .stderr(File::create("zuu/stderr/tests")?)
            .stdout(File::create("zuu/stdout/tests")?)
            .current_dir(".")
            .spawn()
            .expect("test")
            .wait()
            .expect("wait")
            .success()
        {
            results.3 = true;
            assert!(ok(&mut output, "All tests passes", 4).is_ok());
        } else {
            results.3 = false;
            assert!(ko(&mut output, TEST_ERR, 4).is_ok());
        }
        if Command::new("composer")
            .arg("fmt")
            .stderr(File::create("zuu/stderr/fmt")?)
            .stdout(File::create("zuu/stdout/fmt")?)
            .current_dir(".")
            .spawn()
            .expect("composer")
            .wait()
            .expect("wait")
            .success()
        {
            results.4 = true;
            assert!(ok(&mut output, "Source code format respect stantard", 5).is_ok());
        } else {
            results.4 = false;
            assert!(ko(&mut output, FORMAT_ERR, 5).is_ok());
        }
        if Command::new("composer")
            .arg("outdated")
            .stderr(File::create("zuu/stderr/outdated")?)
            .stdout(File::create("zuu/stdout/outdated")?)
            .current_dir(".")
            .spawn()
            .expect("composer")
            .wait()
            .expect("wait")
            .success()
        {
            results.5 = true;
            assert!(ok(&mut output, "Dependencies are up to date", 6).is_ok());
        } else {
            results.5 = false;
            assert!(ko(&mut output, "Dependencies must be updated", 6).is_ok());
        }
        assert!(execute!(&mut output, Print("\n\n")).is_ok());
        if results.0 && results.1 && results.2 && results.3 && results.4 && results.5 {
            return Ok(());
        }
        Err(Error::other("zuu detect error"))
    }

    fn all(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::License,
            Command::new("make")
                .arg(TARGET_LICENSE)
                .current_dir(".")
                .spawn()
                .expect("license")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(
            Checked::Audit,
            Command::new("make")
                .arg(TARGET_AUDIT)
                .current_dir(".")
                .spawn()
                .expect("audit")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Lint,
            Command::new("make")
                .arg(TARGET_LINT)
                .current_dir(".")
                .spawn()
                .expect("lint")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(
            Checked::Test,
            Command::new("make")
                .arg(TARGET_TEST)
                .current_dir(".")
                .spawn()
                .expect("test")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(
            Checked::Fmt,
            Command::new("make")
                .arg(TARGET_FMT)
                .current_dir(".")
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );
        check(&self.checked)
    }
    pub fn check(&mut self) -> ExitCode {
        match self.language {
            Language::Rust => zuu_exit(&self.rust()),
            Language::Php => zuu_exit(&self.php()),
            Language::Unknown => ExitCode::FAILURE,
            _ => zuu_exit(&self.all()),
        }
    }
}
