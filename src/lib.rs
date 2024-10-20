use std::collections::HashMap;
use std::io::Error;
use std::process::{Command, ExitCode};

pub const FORMAT_ERR: &str =
    "Error: Source code is not formatted correctly. Please run the formatter.";
pub const AUDIT_ERR: &str =
    "Error: Security vulnerabilities detected in the code. Please run a security audit.";
pub const TEST_ERR: &str =
    "Error: Test failures. Some tests did not pass. Please review the test results.";
pub const LINT_ERR: &str =
    "Error: Linting issues detected. Your code does not meet style requirements.";
pub const LICENSE_ERR: &str =
    "Error: License issues detected. Some dependencies may have incompatible licenses.";

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
fn check(x: &HashMap<Checked, bool>) -> Result<(), Error> {
    for (i, v) in x {
        if v.eq(&false) {
            return match i {
                Checked::Fmt => Err(Error::other(FORMAT_ERR)),
                Checked::Audit => Err(Error::other(AUDIT_ERR)),
                Checked::Test => Err(Error::other(TEST_ERR)),
                Checked::License => Err(Error::other(LICENSE_ERR)),
                Checked::Lint => Err(Error::other(LINT_ERR)),
            };
        }
    }
    Ok(())
}
pub struct Zuu {
    checked: HashMap<Checked, bool>,
    language: Language,
}
impl Zuu {
    pub fn new(lang: Language) -> Self {
        Self {
            checked: HashMap::new(),
            language: lang,
        }
    }
    fn fsharp(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn r(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn julia(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn crystal(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    fn groovy(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn scala(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn dart(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn matlab(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn cobol(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn fortran(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    fn nim(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    fn vlang(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn ocaml(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn tcl(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn vhdl(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn kotlin(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn swift(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    fn ruby(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn perl(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn typescript(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn haskell(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn clojure(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn bash(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn objective_c(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn erlang(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn lua(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn elixir(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn php(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn rust(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("cargo")
                .args(["fmt", "--check", "--all"])
                .spawn()
                .expect("cargo not founded")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("cargo")
                .args(["test", "--no-fail-fast"])
                .spawn()
                .expect("cargo not founded")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Audit,
            Command::new("cargo")
                .args(["audit"])
                .spawn()
                .expect("cargo not founded")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(
            Checked::Lint,
            Command::new("cargo")
                .args(["clippy"])
                .spawn()
                .expect("cargo not founded")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(
            Checked::License,
            Command::new("cargo")
                .args(["deny", "check"])
                .spawn()
                .expect("cargo not founded")
                .wait()
                .expect("wait")
                .success(),
        );
        check(&self.checked)
    }

    fn go(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn c(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    fn cpp(&mut self) -> Result<(), Error> {
        self.c()
    }

    fn d(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    // Ajout d'une fonction pour Python
    fn python(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }

    pub fn nodejs(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    fn java(&mut self) -> Result<(), Error> {
        self.checked.insert(
            Checked::Fmt,
            Command::new("fantomas")
                .args(["--check", "*.fs"])
                .spawn()
                .expect("fantomas")
                .wait()
                .expect("wait")
                .success(),
        );

        self.checked.insert(
            Checked::Test,
            Command::new("dotnet")
                .args(["test"])
                .spawn()
                .expect("dotnet")
                .wait()
                .expect("wait")
                .success(),
        );
        self.checked.insert(Checked::License, true);
        self.checked.insert(Checked::Audit, true);
        self.checked.insert(
            Checked::Lint,
            Command::new("fsharplint")
                .args(["--check", "*.fs"]) // Vérifie le linting
                .spawn()
                .expect("fsharplint")
                .wait()
                .expect("msg")
                .success(),
        );
        check(&self.checked)
    }
    pub fn check(&mut self) -> ExitCode {
        match self.language {
            Language::Rust => zuu_exit(&self.rust()),
            Language::Go => zuu_exit(&self.go()),
            Language::C => zuu_exit(&self.c()),
            Language::Cpp => zuu_exit(&self.cpp()),
            Language::D => zuu_exit(&self.d()),
            Language::Python => zuu_exit(&self.python()),
            Language::Php => zuu_exit(&self.php()),
            Language::Java => zuu_exit(&self.java()),
            Language::Kotlin => zuu_exit(&self.kotlin()),
            Language::Swift => zuu_exit(&self.swift()),
            Language::Ruby => zuu_exit(&self.ruby()),
            Language::Perl => zuu_exit(&self.perl()),
            Language::Scala => zuu_exit(&self.scala()),
            Language::TypeScript => zuu_exit(&self.typescript()),
            Language::Elixir => zuu_exit(&self.elixir()),
            Language::Haskell => zuu_exit(&self.haskell()),
            Language::Clojure => zuu_exit(&self.clojure()),
            Language::Bash => zuu_exit(&self.bash()),
            Language::ObjectiveC => zuu_exit(&self.objective_c()),
            Language::Erlang => zuu_exit(&self.erlang()),
            Language::Lua => zuu_exit(&self.lua()),
            Language::FSharp => zuu_exit(&self.fsharp()),
            Language::R => zuu_exit(&self.r()),
            Language::Julia => zuu_exit(&self.julia()),
            Language::Crystal => zuu_exit(&self.crystal()),
            Language::Groovy => zuu_exit(&self.groovy()),
            Language::Dart => zuu_exit(&self.dart()),
            Language::Matlab => zuu_exit(&self.matlab()),
            Language::Cobol => zuu_exit(&self.cobol()),
            Language::Fortran => zuu_exit(&self.fortran()),
            Language::Nim => zuu_exit(&self.nim()),
            Language::Nodejs => zuu_exit(&self.nodejs()),
            Language::Vlang => zuu_exit(&self.vlang()),
            Language::OCaml => zuu_exit(&self.ocaml()),
            Language::Tcl => zuu_exit(&self.tcl()),
            Language::VHDL => zuu_exit(&self.vhdl()),
            Language::Unknown => ExitCode::FAILURE,
        }
    }
}
