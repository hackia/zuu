use std::env::var;
use std::io::{Error, ErrorKind};
use std::process::Command;

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

pub struct Zuu {
    test: bool,
    format: bool,
    lint: bool,
    audit: bool,
    license: bool,
}

impl Zuu {
    pub fn new() -> Self {
        Self {
            test: var("TESTS")
                .unwrap_or(String::from("false"))
                .parse::<bool>()
                .unwrap_or(false),
            format: var("FORMAT")
                .unwrap_or(String::from("false"))
                .parse::<bool>()
                .unwrap_or(false),
            lint: var("LINT")
                .unwrap_or(String::from("false"))
                .parse::<bool>()
                .unwrap_or(false),
            audit: var("AUDIT")
                .unwrap_or(String::from("false"))
                .parse::<bool>()
                .unwrap_or(false),
            license: var("LICENSE")
                .unwrap_or(String::from("false"))
                .parse::<bool>()
                .unwrap_or(false),
        }
    }
    fn fsharp(self) -> Result<(), Error> {
        if self.format
            && Command::new("fantomas")
                .args(&["--check", "*.fs"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: fantomas . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("dotnet")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: dotnet test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual security checks are required for F#.", AUDIT_ERR),
            ));
        }

        if self.lint
            && Command::new("fsharplint")
                .args(&["--check", "*.fs"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: fsharplint *.fs to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dotnet licenses to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn r(self) -> Result<(), Error> {
        if self.format
            && Command::new("Rscript")
                .args(&["-e", "styler::style_file(\".\")"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: styler::style_file() to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("Rscript")
                .args(&["-e", "testthat::test_dir(\"tests\")"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: testthat::test_dir(\"tests\") to see test failures.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual security checks are required for R.", AUDIT_ERR),
            ));
        }

        if self.lint
            && Command::new("Rscript")
                .args(&["-e", "lintr::lint_dir()"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: lintr::lint_dir() to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks required for R packages.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn julia(self) -> Result<(), Error> {
        if self.format
            && Command::new("julia")
                .args(&["-e", "using JuliaFormatter; format(\".\")"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: using JuliaFormatter; format(\".\") to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("julia")
                .args(&["-e", "Pkg.test()"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: Pkg.test() to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual security checks required for Julia.", AUDIT_ERR),
            ));
        }

        if self.lint
            && Command::new("julia")
                .args(&["-e", "using Lint; lintdir(\".\")"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: using Lint; lintdir(\".\") to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks required for Julia packages.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn crystal(self) -> Result<(), Error> {
        if self.format
            && Command::new("crystal")
                .args(&["tool", "format", "--check"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: crystal tool format to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("crystal")
                .args(&["spec"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: crystal spec to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks required for Crystal.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("ameba")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: ameba . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks required for Crystal packages.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn java(self) -> Result<(), Error> {
        if self.format
            && Command::new("google-java-format")
                .args(&["--dry-run", "-n", "*.java"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: google-java-format to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("mvn")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: mvn test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("spotbugs")
                .args(&["-textui", "*.java"]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: spotbugs to check for security vulnerabilities in Java code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("checkstyle")
                .args(&["-c", "/google_checks.xml", "*.java"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: checkstyle -c /google_checks.xml to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("mvn")
                .args(&["license:check"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: mvn license:check to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }
    fn groovy(self) -> Result<(), Error> {
        if self.format
            && Command::new("groovyfmt")
                .args(&["--check", "*.groovy"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: groovyfmt to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("gradle")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: gradle test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("spotbugs")
                .args(&["."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: spotbugs . to check for security vulnerabilities.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("codenarc")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: codenarc . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("gradle")
                .args(&["licenseReport"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: gradle licenseReport to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn scala(self) -> Result<(), Error> {
        if self.format
            && Command::new("scalafmt")
                .args(&["--test", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: scalafmt --test . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("sbt")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: sbt test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("findbugs")
                .args(&["."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: findbugs to check for security vulnerabilities in Scala code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("scalastyle")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: scalastyle . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("sbt")
                .args(&["licenseReport"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: sbt licenseReport to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn dart(self) -> Result<(), Error> {
        if self.format
            && Command::new("dart")
                .args(&["format", "--set-exit-if-changed", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dart format to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("dart")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: dart test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Dart.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("dart")
                .args(&["analyze"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dart analyze to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for Dart.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn matlab(self) -> Result<(), Error> {
        if self.format {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nThere is no standard tool for automatic formatting in MATLAB.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("matlab")
                .args(&["-batch", "runtests"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: runtests to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for MATLAB.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("matlab")
                .args(&["-batch", "checkcode ."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: checkcode . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for MATLAB.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn cobol(self) -> Result<(), Error> {
        if self.format {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual formatting is required for COBOL.", FORMAT_ERR),
            ));
        }

        if self.test
            && Command::new("cobc")
                .args(&["-x", "main.cob"]) // Compile COBOL et exécute les tests
                .status()
                .is_err()
            && Command::new("./main").status().is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cobc -x main.cob and ./main to run tests.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for COBOL.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLinting is not commonly done in COBOL. Manual review required.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for COBOL.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn fortran(self) -> Result<(), Error> {
        if self.format
            && Command::new("fprettify")
                .args(&["--check", "*.f90"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: fprettify --check *.f90 to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("pfunit")
                .args(&["-run"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: pfunit -run to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Fortran.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("fortlint")
                .args(&["*.f90"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: fortlint *.f90 to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for Fortran.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }
    fn nim(self) -> Result<(), Error> {
        if self.format
            && Command::new("nimpretty")
                .args(&["--check", "*.nim"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: nimpretty --check *.nim to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("nimble")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: nimble test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Nim.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("nim")
                .args(&["check", "*.nim"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: nim check *.nim to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for Nim.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }
    fn vlang(self) -> Result<(), Error> {
        if self.format
            && Command::new("v")
                .args(&["fmt", "-verify"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: v fmt -verify to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("v")
                .args(&["test", "."]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: v test . to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual security checks are required for V.", AUDIT_ERR),
            ));
        }

        if self.lint {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLinting is part of the V build system. No additional checks needed.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual license checks are required for V.", LICENSE_ERR),
            ));
        }

        Ok(())
    }

    fn ocaml(self) -> Result<(), Error> {
        if self.format
            && Command::new("ocamlformat")
                .args(&["--check", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: ocamlformat . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("dune")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: dune test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for OCaml.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("ocamllint")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: ocamllint . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for OCaml.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn tcl(self) -> Result<(), Error> {
        if self.format {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual formatting is required for Tcl.", FORMAT_ERR),
            ));
        }

        if self.test
            && Command::new("tclsh")
                .args(&["tests.tcl"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: tclsh tests.tcl to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Tcl.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual linting required for Tcl.", LINT_ERR),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for Tcl.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn vhdl(self) -> Result<(), Error> {
        if self.format {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nManual formatting is required for VHDL.", FORMAT_ERR),
            ));
        }

        if self.test
            && Command::new("ghdl")
                .args(&["-r", "testbench"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: ghdl -r testbench to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for VHDL.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("vhdl-lint")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: vhdl-lint . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual license checks are required for VHDL.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn kotlin(self) -> Result<(), Error> {
        if self.format
            && Command::new("ktlint")
                .args(&["--check", "*.kt"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: ktlint --check *.kt to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }
        if self.test
            && Command::new("gradle")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: gradle test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("spotbugs")
                .args(&["*.kt"]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: spotbugs to check for security vulnerabilities in Kotlin code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("detekt")
                .args(&["--input", "."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: detekt to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("gradle")
                .args(&["license"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: gradle license to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn swift(self) -> Result<(), Error> {
        if self.format
            && Command::new("swiftformat")
                .args(&["--lint", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: swiftformat . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("swift")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: swift test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("swiftlint")
                .args(&["analyze"]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                "{}\nRun: swiftlint analyze to check for security vulnerabilities in Swift code.",
                AUDIT_ERR
            ),
            ));
        }

        if self.lint
            && Command::new("swiftlint")
                .args(&["lint"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: swiftlint lint to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("license_finder")
                .args(&["."]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: license_finder to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }
    fn ruby(self) -> Result<(), Error> {
        if self.format
            && Command::new("rubocop")
                .args(&["--auto-correct"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: rubocop --auto-correct to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test && Command::new("rspec").status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: rspec to see test failures.", TEST_ERR),
            ));
        }

        if self.audit && Command::new("brakeman").status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: brakeman to check for security vulnerabilities in Ruby code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint && Command::new("rubocop").status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: rubocop to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license && Command::new("license_finder").status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: license_finder to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn perl(self) -> Result<(), Error> {
        if self.format
            && Command::new("perltidy")
                .args(&["-b", "*.pl"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: perltidy *.pl to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test && Command::new("prove").status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: prove to see test failures.", TEST_ERR),
            ));
        }

        if self.lint
            && Command::new("perlcritic")
                .args(&["*.pl"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: perlcritic *.pl to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("licensecheck")
                .args(&["*.pl"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: licensecheck *.pl to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn typescript(self) -> Result<(), Error> {
        if self.format
            && Command::new("prettier")
                .args(&["--check", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: prettier --check . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("npm")
                .args(&["run", "test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: npm run test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("npm")
                .args(&["audit"]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: npm audit to check for security vulnerabilities in TypeScript code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("eslint")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: eslint . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("license-checker")
                .args(&["--production"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: license-checker --production to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn haskell(self) -> Result<(), Error> {
        if self.format
            && Command::new("stylish-haskell")
                .args(&["-r", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: stylish-haskell -r . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("stack")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: stack test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("liquidhaskell")
                .args(&["."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: liquidhaskell . to check for security vulnerabilities.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("hlint")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: hlint . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("cabal")
                .args(&["check"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cabal check to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn clojure(self) -> Result<(), Error> {
        if self.format
            && Command::new("cljfmt")
                .args(&["check"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cljfmt fix to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("lein")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: lein test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Clojure.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("lein")
                .args(&["kibit"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: lein kibit to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("lein")
                .args(&["licenses"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: lein licenses to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn bash(self) -> Result<(), Error> {
        if self.format
            && Command::new("shfmt")
                .args(&["-d", "."]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: shfmt -w . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("bats")
                .args(&["."]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: bats . to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("shellcheck")
                .args(&["."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: shellcheck . to check for security vulnerabilities.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("shellcheck")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: shellcheck . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLicense checks for Bash scripts must be done manually.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn objective_c(self) -> Result<(), Error> {
        if self.format
            && Command::new("clang-format")
                .args(&["-i", "*.m", "*.h"]) // Vérifie et corrige le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: clang-format -i *.m *.h to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("xcodebuild")
                .args(&["test", "-scheme", "YourScheme"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: xcodebuild test -scheme YourScheme to see test failures.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit
            && Command::new("oclint")
                .args(&["."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: oclint . to check for security vulnerabilities in Objective-C code.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("oclint")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: oclint . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nLicense checks for Objective-C must be done manually via CocoaPods or Carthage.",
                LICENSE_ERR
            )));
        }

        Ok(())
    }

    fn erlang(self) -> Result<(), Error> {
        if self.format
            && Command::new("erl_tidy")
                .args(&["."]) // Vérifie et corrige le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: erl_tidy . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("rebar3")
                .args(&["ct"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: rebar3 ct to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Erlang.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLinting in Erlang is typically done via manual reviews or external tools.",
                    LINT_ERR
                ),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLicense checks for Erlang must be done manually.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn lua(self) -> Result<(), Error> {
        if self.format
            && Command::new("lua-fmt")
                .args(&["-c", "*.lua"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: lua-fmt -c *.lua to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("busted")
                .args(&["."]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: busted . to see test failures.", TEST_ERR),
            ));
        }

        if self.audit {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nManual security checks are required for Lua.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint
            && Command::new("luacheck")
                .args(&["."]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: luacheck . to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nLicense checks for Lua must be done manually.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn elixir(self) -> Result<(), Error> {
        if self.format
            && Command::new("mix")
                .args(&["format", "--check-formatted"]) // Vérifie le formatage
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: mix format to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("mix")
                .args(&["test"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: mix test to see test failures.", TEST_ERR),
            ));
        }

        if self.audit
            && Command::new("sobelow")
                .args(&["--config", "."]) // Vérifie les vulnérabilités
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
            "{}\nRun: sobelow --config . to check for security vulnerabilities in Elixir code.",
            AUDIT_ERR
        ),
            ));
        }

        if self.lint
            && Command::new("mix")
                .args(&["credo"]) // Vérifie le linting
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!("{}\nRun: mix credo to check for linting issues.", LINT_ERR),
            ));
        }

        if self.license
            && Command::new("mix")
                .args(&["hex.license_report"]) // Vérifie les licences
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: mix hex.license_report to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn php(self) -> Result<(), Error> {
        if self.format
            && Command::new("php-cs-fixer")
                .args(&["fix", "--dry-run", "--diff"]) // Vérifie si le code est bien formaté
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: php-cs-fixer fix to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("phpunit")
                .args(&["--stop-on-failure"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: phpunit --stop-on-failure to see test failures.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit
            && Command::new("phpstan")
                .args(&["analyse", "."]) // Vérifie les vulnérabilités et erreurs
                .status()
                .is_err()
        {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nRun: phpstan analyse . to check for security vulnerabilities and static analysis in the PHP code.",
                AUDIT_ERR
            )));
        }

        if self.lint
            && Command::new("phpmd")
                .args(&[
                    ".",
                    "text",
                    "cleancode,codesize,controversial,design,naming,unusedcode",
                ]) // Vérifie le style de code PHP
                .status()
                .is_err()
        {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nRun: phpmd . text cleancode,codesize,controversial,design,naming,unusedcode to check for linting issues in the PHP code.",
                LINT_ERR
            )));
        }

        if self.license
            && Command::new("composer")
                .args(&["licenses"]) // Vérifie les licences des dépendances PHP
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                "{}\nRun: composer licenses to check for license issues in the PHP dependencies.",
                LICENSE_ERR
            ),
            ));
        }

        Ok(())
    }

    fn rust(self) -> Result<(), Error> {
        if self.format
            && Command::new("cargo")
                .args(&["fmt", "--all", "--", "--check"])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cargo fmt --all -- --check to see the formatting issues.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("cargo")
                .args(&["test", "--", "--no-fail-fast"])
                .status()
                .is_err()
        {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nRun: cargo test -- --no-fail-fast to run all tests and see detailed test failures.",
                TEST_ERR
            )));
        }

        if self.audit && Command::new("cargo").args(&["audit"]).status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cargo audit to see the security vulnerabilities.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint && Command::new("cargo").args(&["clippy"]).status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cargo clippy to view the linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("cargo")
                .args(&["deny", "check"])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: cargo deny check to verify the license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }

    fn go(self) -> Result<(), Error> {
        if self.format && Command::new("gofmt").args(&["-l", "."]).status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: gofmt -l . to see the formatting issues.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("go")
                .args(&["test", "./..."])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: go test ./... to run all Go tests and see detailed test failures.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit && Command::new("go").args(&["vet", "./..."]).status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: go vet ./... to check for audit issues.",
                    AUDIT_ERR
                ),
            ));
        }

        if self.lint && Command::new("golint").args(&["./..."]).status().is_err() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: golint ./... to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("licensecheck")
                .args(&["*.go"])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: licensecheck *.go to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }
        Ok(())
    }

    fn c(self) -> Result<(), Error> {
        if Command::new("cmake").arg(".").status().is_ok() {
            if self.format
                && Command::new("make")
                    .arg("fmt")
                    .current_dir(".")
                    .status()
                    .is_err()
            {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{}\n:make fmt detect errors", FORMAT_ERR),
                ));
            }
            if self.audit
                && Command::new("make")
                    .arg("audit")
                    .current_dir(".")
                    .status()
                    .is_err()
            {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{}\n:make audit detect errors", FORMAT_ERR),
                ));
            }
            if self.lint
                && Command::new("make")
                    .arg("lint")
                    .current_dir(".")
                    .status()
                    .is_err()
            {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{}\n:make lint detect errors", FORMAT_ERR),
                ));
            }
            if self.license
                && Command::new("make")
                    .arg("license")
                    .current_dir(".")
                    .status()
                    .is_err()
            {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("{}\n:make license detect errors", FORMAT_ERR),
                ));
            }
            return Ok(());
        }
        Err(Error::new(ErrorKind::NotFound, " CMakeLists.txt not found"))
    }

    fn cpp(self) -> Result<(), Error> {
        self.c()
    }

    fn d(self) -> Result<(), Error> {
        if self.format
            && Command::new("dfmt")
                .args(&["--check", "."])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dfmt --check . to see the formatting issues.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("dub")
                .args(&["test", "--coverage"])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dub test --coverage to run tests with coverage for D.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit
            && Command::new("dscanner")
                .args(&["--security", "."])
                .status()
                .is_err()
        {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nRun: dscanner --security . to check for security vulnerabilities in the D code.",
                AUDIT_ERR
            )));
        }

        if self.lint
            && Command::new("dscanner")
                .args(&["--styleCheck", "."])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: dscanner --styleCheck . to check for linting issues.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("licensecheck")
                .args(&["*.d"])
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: licensecheck *.d to verify license issues.",
                    LICENSE_ERR
                ),
            ));
        }

        Ok(())
    }
    // Ajout d'une fonction pour Python
    fn python(self) -> Result<(), Error> {
        if self.format
            && Command::new("black")
                .args(&["--check", "."]) // Vérifie si le code est bien formaté
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: black . to apply formatting corrections.",
                    FORMAT_ERR
                ),
            ));
        }

        if self.test
            && Command::new("pytest")
                .args(&["--maxfail=1", "--disable-warnings"]) // Exécute les tests
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: pytest --maxfail=1 --disable-warnings to see test failures.",
                    TEST_ERR
                ),
            ));
        }

        if self.audit
            && Command::new("bandit")
                .args(&["-r", "."]) // Vérifie les vulnérabilités de sécurité
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                "{}\nRun: bandit -r . to check for security vulnerabilities in the Python code.",
                AUDIT_ERR
            ),
            ));
        }

        if self.lint
            && Command::new("flake8")
                .args(&["."]) // Vérifie le style de code Python
                .status()
                .is_err()
        {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "{}\nRun: flake8 . to check for linting issues in the Python code.",
                    LINT_ERR
                ),
            ));
        }

        if self.license
            && Command::new("pip-licenses")
                .args(&["--from=mixed", "--with-urls", "--format=markdown"]) // Vérifie les licences des dépendances Python
                .status()
                .is_err()
        {
            return Err(Error::new(ErrorKind::Other, format!(
                "{}\nRun: pip-licenses --from=mixed --with-urls --format=markdown to check for license issues in the Python dependencies.",
                LICENSE_ERR
            )));
        }

        Ok(())
    }

    pub fn nodejs(self) -> Result<(), Error> {
        if self.format
            && Command::new("prettier")
                .args(&["--check", "."])
                .status()
                .is_err()
        {
            Err(Error::new(ErrorKind::Other, FORMAT_ERR))
        } else if self.test && Command::new("npm").args(&["test"]).status().is_err() {
            Err(Error::new(ErrorKind::Other, TEST_ERR))
        } else if self.lint && Command::new("eslint").args(&["."]).status().is_err() {
            Err(Error::new(ErrorKind::Other, LINT_ERR))
        } else if self.audit && Command::new("npm").args(&["audit"]).status().is_err() {
            Err(Error::new(ErrorKind::Other, AUDIT_ERR))
        } else {
            Ok(())
        }
    }
    pub fn run(self, lang: &Language) -> Result<(), Error> {
        match lang {
            Language::Rust => self.rust(),
            Language::Go => self.go(),
            Language::C => self.c(),
            Language::Cpp => self.cpp(),
            Language::D => self.d(),
            Language::Python => self.python(),
            Language::Php => self.php(),
            Language::Java => self.java(),
            Language::Kotlin => self.kotlin(),
            Language::Swift => self.swift(),
            Language::Ruby => self.ruby(),
            Language::Perl => self.perl(),
            Language::Scala => self.scala(),
            Language::TypeScript => self.typescript(),
            Language::Elixir => self.elixir(),
            Language::Haskell => self.haskell(),
            Language::Clojure => self.clojure(),
            Language::Bash => self.bash(),
            Language::ObjectiveC => self.objective_c(),
            Language::Erlang => self.erlang(),
            Language::Lua => self.lua(),
            Language::FSharp => self.fsharp(),
            Language::R => self.r(),
            Language::Julia => self.julia(),
            Language::Crystal => self.crystal(),
            Language::Groovy => self.groovy(),
            Language::Dart => self.dart(),
            Language::Matlab => self.matlab(),
            Language::Cobol => self.cobol(),
            Language::Fortran => self.fortran(),
            Language::Nim => self.nim(),
            Language::Nodejs => self.nodejs(),
            Language::Vlang => self.vlang(),
            Language::OCaml => self.ocaml(),
            Language::Tcl => self.tcl(),
            Language::VHDL => self.vhdl(),
            Language::Unknown => Err(Error::new(
                ErrorKind::Other,
                "Error: Unknown language. Please specify a supported language.",
            )),
        }
    }
}
