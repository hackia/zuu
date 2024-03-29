use crate::helpers::{ko, ok, okay, run};
use crate::Language::{Cmake, Composer, Docker, Go, Make, Rust, Unknown};
use chrono::{Datelike, Local, Timelike};
use std::collections::HashMap;
use std::env::{args, current_dir};
use std::fs;
use std::fs::File;
use std::io::{read_to_string, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::process::exit;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tabled::builder::Builder;
use tabled::settings::Style;

pub const SUCCESS: &str = "Success";
pub const FAILURE: &str = "Failure";
pub const OUTPUT: &str = "zuu";
pub const STDOUT: &str = "stdout";
pub const STDERR: &str = "stderr";

pub mod helpers;

pub enum Language {
    Rust,
    Go,
    Docker,
    Make,
    Composer,
    Cmake,
    Unknown,
}

///
///
/// # Panics
///
#[must_use]
fn make(cmd: &str) -> i32 {
    run("make", cmd, output(cmd, true), true)
}

#[must_use]
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(File::open(filename).expect("failed to found"))
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn check_make(started: Instant) -> i32 {
    let mut cmd: Vec<String> = Vec::new();
    for x in &read_lines("Makefile") {
        if x.starts_with("all") {
            cmd.push("all".to_string());
        }
        if x.starts_with("install") {
            cmd.push("install".to_string());
        }
        if x.starts_with("dist") {
            cmd.push("dist".to_string());
        }
        if x.starts_with("clean") {
            cmd.push("clean".to_string());
        }

        if x.starts_with("uninstall") {
            cmd.push("uninstall".to_string());
        }
        if x.starts_with("install") {
            cmd.push("install".to_string());
        }

        if x.starts_with("install-strip") {
            cmd.push("install-strip".to_string());
        }
        if x.starts_with("distclean") {
            cmd.push("distclean".to_string());
        }

        if x.starts_with("install") {
            cmd.push("install".to_string());
        }

        if x.starts_with("maintainer-clean") {
            cmd.push("maintainer-clean".to_string());
        }
        if x.starts_with("mostlyclean") {
            cmd.push("mostlyclean".to_string());
        }
    }
    for x in &cmd {
        assert!(make(started, x.as_str()).eq(&0));
    }
    0
}

fn check_cmake(started: Instant) -> i32 {
    if run(
        "Started",
        "Cmake",
        "cmake",
        ".",
        "Makefile created successfully",
        "Failed to create Makefile",
        started,
    )
    .eq(&0)
        && run(
            "Started",
            "Make",
            "make",
            "",
            "Project built successfully",
            "Failed to built the project",
            started,
        )
        .eq(&0)
        && run(
            "Started",
            "Install",
            "make",
            "install",
            "Project installed successfully",
            "Failed to install the project",
            started,
        )
        .eq(&0)
    {
        return 0;
    }
    1
}

fn check_rust(started: Instant) -> i32 {
    let project = run(
        "Started",
        "Project",
        "cargo",
        "verify-project",
        "verify-project no detect errors",
        "verify-project detect errors",
        Instant::now(),
    );
    let bench = run(
        "Started",
        "Project",
        "cargo",
        "bench --no-fail-fast --all-targets --message-format human",
        "bench no detect errors",
        "bench detect errors",
        Instant::now(),
    );
    let build = run(
        "Started",
        "Build",
        "cargo",
        "build --all-targets --release -j 4 --future-incompat-report",
        "Build no detect errors",
        "Build detect errors",
        Instant::now(),
    );
    let audit = run(
        "Started",
        "Audit",
        "cargo",
        "audit",
        "Audit no detect errors",
        "Audit detect errors",
        Instant::now(),
    );

    let clippy = run("Started",
                     "Clippy",
                     "cargo",
                     "clippy -- -F keyword_idents -F warnings -F let-underscore -F rust-2018-compatibility -F rust-2018-idioms  -F rust-2021-compatibility -F future-incompatible -F unused -F unused_crate_dependencies -F unused_extern_crates -F unused_macro_rules -F unused_results -F unused_qualifications -F nonstandard-style -F macro_use_extern_crate -F absolute_paths_not_starting_with_crate -F ambiguous_glob_imports -F clippy::all -F clippy::perf -F clippy::pedantic -F clippy::style -F clippy::suspicious -F clippy::correctness -F clippy::nursery -F clippy::complexity -F clippy::cargo",
                     "Your code is correct",
                     "Your code is incorrect",
                     Instant::now());
    let tests = run(
        "Started",
        "Tests",
        "cargo",
        "test --all-targets --all-features --release -j 4 --no-fail-fast",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    let checkup = run(
        "Started",
        "Check",
        "cargo",
        "check --all-targets --release --message-format human -j 4",
        "Your code is correct",
        "Your code is incorrect",
        Instant::now(),
    );
    let format = run(
        "Started",
        "Format",
        "cargo",
        "fmt --check",
        "Your code is formatted correctness",
        "Your project is bad formatted",
        Instant::now(),
    );
    let install = run(
        "Started",
        "Install",
        "cargo",
        "install --path . --no-track --bins --examples --all-features -j 4 --force",
        "Your project can be installed",
        "Your project cannot be installed",
        Instant::now(),
    );
    if format.eq(&0)
        && checkup.eq(&0)
        && tests.eq(&0)
        && clippy.eq(&0)
        && audit.eq(&0)
        && project.eq(&0)
        && bench.eq(&0)
        && build.eq(&0)
        && install.eq(&0)
    {
        ok("Your code can be committed", started);
        status();
        0
    } else {
        ko("Your code contains failures", started);
        status();
        1
    }
}

fn check_go(started: Instant) -> i32 {
    let verify = run(
        "Started",
        "Verify",
        "go",
        "mod verify",
        "Your code can it's verified successfully",
        "Your project is not valid",
        Instant::now(),
    );
    let build = run(
        "Started",
        "Build",
        "go",
        "build",
        "Your code can be built",
        "Your project cannot be built",
        Instant::now(),
    );
    let tests = run(
        "Started",
        "Test",
        "go",
        "test -v",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    let vet = run(
        "Started",
        "Test",
        "go",
        "vet",
        "No test failures",
        "Test have failures",
        Instant::now(),
    );
    if vet.eq(&0) && tests.eq(&0) && tests.eq(&0) && verify.eq(&0) && build.eq(&0) {
        ok("Your code can be committed", started);
        status();
        0
    } else {
        ko("Your code contains failures", started);
        status();
        1
    }
}

fn docker(s: Instant) -> i32 {
    let x = run(
        "Started",
        "Docker",
        "docker-compose",
        "up",
        "Your code can be committed",
        "Your code contains failures",
        s,
    );
    let _ = run(
        "Closing",
        "Docker",
        "docker-compose",
        "down",
        "Your code can be committed",
        "Your code contains failures",
        s,
    );

    match x {
        0 => 0,
        _ => 1,
    }
}

fn check(language: &Language, s: Instant) -> i32 {
    match language {
        Rust => check_rust(s),
        Go => check_go(s),
        Docker => docker(s),
        Make => check_make(s),
        Composer => check_composer(s),
        Cmake => check_cmake(s),
        Unknown => {
            ko("Language not supported", s);
            1
        }
    }
}

fn detect() -> &'static Language {
    for (f, l) in &all() {
        if Path::new(f.as_str()).exists() {
            return l;
        }
    }
    &Unknown
}

#[must_use]
fn php(f: &str, command: &str) -> i32 {
    if Path::new(f).exists() {
        return run("php", command, output(command, true), true);
    }
    1
}
fn spin(b: &str, data: &str) {
    let i = ["|", "/", "-", "\\", "|"];
    for &x in &i {
        print!("\r\x1b[1;37m {x}\x1b[1;32m  Sleeping\x1b[0m");
        std::io::stdout().flush().expect("a");
        sleep(Duration::from_millis(100));
    }
    std::io::stdout().flush().expect("a");
    print!("\r\x1b[1;37m *\x1b[1;32m   {b} \x1b[1;37m{data}\x1b[0m");
    std::io::stdout().flush().expect("a");
}

fn output(filename: &str, stdout: bool) -> File {
    if !Path::new(OUTPUT).exists() {
        fs::create_dir(OUTPUT).expect("Fail to create the output directory");
        fs::create_dir(format!("{OUTPUT}{MAIN_SEPARATOR}{STDERR}").as_str())
            .expect("Fail to create the output directory");
        fs::create_dir(format!("{OUTPUT}{MAIN_SEPARATOR}{STDOUT}").as_str())
            .expect("Fail to create the output directory");
    }
    if stdout {
        File::create(format!("{OUTPUT}{MAIN_SEPARATOR}{STDOUT}{MAIN_SEPARATOR}{filename}").as_str())
            .expect("")
    } else {
        File::create(format!("{OUTPUT}{MAIN_SEPARATOR}{STDERR}{MAIN_SEPARATOR}{filename}").as_str())
            .expect("")
    }
}

pub struct Zuu {
    started: Instant,
    args: Vec<String>,
}
impl Zuu {
    ///
    /// # Panics
    ///
    #[must_use]
    pub fn new(x: Instant, args: Vec<String>) -> Self {
        Self { started: x, args }
    }
    pub fn waiting(&mut self, code: i32) {
        print!("{}", ansi_escapes::CursorHide);
        okay("Press Ctrl+c to quit");
        if code.eq(&0) {
            for _t in 1..61 {
                spin("Success", "Your code can be committed");
            }
        } else {
            for _t in 1..61 {
                spin("Failure", "Your code contains failures");
            }
        }
    }

    ///
    /// # Panics
    ///
    pub fn init(&mut self) -> i32 {
        let git_hook_content = "#!/bin/sh\nzuu\n exit $?";
        let hg_hook_content = "[hooks]\nprecommit = zuu";
        if Path::new(".git").exists() {
            let mut f = File::create(
                format!(".git{MAIN_SEPARATOR}hooks{MAIN_SEPARATOR}pre-commit").as_str(),
            )
            .expect("failed to create the hook file");
            f.write_all(git_hook_content.as_bytes())
                .expect("failed to write content");
            f.sync_all().expect("failed to sync data");
            okay("Don't forget to add the execution right for the .git/hooks/pre-commit file");
            okay("Your project it's now tracked by zuu");
        }
        if Path::new(".hg").exists() {
            let mut f = File::create(format!(".hg{MAIN_SEPARATOR}hgrc").as_str())
                .expect("failed to create the hook file");
            f.write_all(hg_hook_content.as_bytes())
                .expect("failed to write content");
            f.sync_all().expect("failed to sync data");
            ok("Your project it's now tracked by zuu", self.started);
            exit(0);
        }

        ko("Versioning must be git or mercurial", self.started);
        exit(1);
    }

    ///
    /// # Panics
    ///
    pub fn all(&mut self) -> HashMap<String, &'static Language> {
        let mut all: HashMap<String, &'static Language> = HashMap::new();
        assert!(all.insert(String::from("compose.yaml"), &Docker).is_none());
        assert!(all.insert(String::from("Cargo.toml"), &Rust).is_none());
        assert!(all.insert(String::from("go.mod"), &Go).is_none());
        assert!(all.insert(String::from("Makefile"), &Make).is_none());
        assert!(all
            .insert(String::from("composer.json"), &Composer)
            .is_none());
        assert!(all.insert(String::from("CMakeLists.txt"), &Cmake).is_none());
        all
    }
    #[must_use]
    pub fn detect(&mut self) -> &'static Language {
        for (f, &l) in &self.all() {
            if Path::new(f.as_str()).exists() {
                return l;
            }
        }
        &Unknown
    }

    ///
    /// # Panics
    ///
    #[must_use]
    pub fn go(&mut self) -> i32 {
        let verify = run("go", "mod verify", output("verify", true), true);
        let build = run("go", "build", output("build", true), true);
        let tests = run("go", "test -v", output("tests", true), true);
        let vet = run("go", "vet", output("vet", true), true);
        if vet.eq(&0) && tests.eq(&0) && tests.eq(&0) && verify.eq(&0) && build.eq(&0) {
            0
        } else {
            1
        }
    }

    ///
    /// a
    ///
    /// # Panics
    ///
    #[must_use]
    pub fn composer(&mut self) -> i32 {
        assert!(run("composer", "install", output("install", true), true).eq(&0));
        let audit = run("composer", "audit", output("audit", true), true);
        let diagnose = run("composer", "diagnose", output("diagnose", true), true);
        let tests = php(
            "phpunit.xml",
            format!("vendor{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}phpunit").as_str(),
        );
        let checkup = php(
            "phpstan.neon",
            format!("vendor{MAIN_SEPARATOR}bin{MAIN_SEPARATOR}phpstan").as_str(),
        );

        if tests.eq(&0) && checkup.eq(&0) && audit.eq(&0) && diagnose.eq(&0) {
            return 0;
        }
        1
    }
    #[must_use]
    pub fn cmake(&mut self) -> i32 {
        if run("cmake", ".", output("cmake", true), true).eq(&0)
            && run("make", "", output("make", true), true).eq(&0)
            && run("make", "install", output("install", true), true).eq(&0)
        {
            return 0;
        }
        1
    }
    #[must_use]
    pub fn docker(&mut self) -> i32 {
        let x = run("docker-compose", "up", output("docker", true), true);
        let _ = run("docker-compose", "down", output("docker-down", true), true);

        match x {
            -1 => 0,
            _ => 1,
        }
    }

    ///
    /// # Panics
    ///
    #[must_use]
    pub fn make(&mut self) -> i32 {
        let mut cmd: Vec<String> = Vec::new();
        for x in &read_lines("Makefile") {
            if x.starts_with("all") {
                cmd.push("all".to_string());
            }
            if x.starts_with("install") {
                cmd.push("install".to_string());
            }
            if x.starts_with("dist") {
                cmd.push("dist".to_string());
            }
            if x.starts_with("clean") {
                cmd.push("clean".to_string());
            }

            if x.starts_with("uninstall") {
                cmd.push("uninstall".to_string());
            }
            if x.starts_with("install") {
                cmd.push("install".to_string());
            }

            if x.starts_with("install-strip") {
                cmd.push("install-strip".to_string());
            }
            if x.starts_with("distclean") {
                cmd.push("distclean".to_string());
            }

            if x.starts_with("install") {
                cmd.push("install".to_string());
            }

            if x.starts_with("maintainer-clean") {
                cmd.push("maintainer-clean".to_string());
            }
            if x.starts_with("mostlyclean") {
                cmd.push("mostlyclean".to_string());
            }
        }
        for x in &cmd {
            assert!(make(x.as_str()).eq(&0));
        }
        0
    }

    pub fn date(&mut self) -> String {
        let t = Local::now();
        format!(
            "{}-{}-{} {}:{}:{}",
            t.year(),
            t.month(),
            t.day(),
            t.hour(),
            t.minute(),
            t.second()
        )
    }

    pub fn filename(&mut self, f: &str, stdout: bool) -> String {
        if stdout {
            format!("{OUTPUT}{MAIN_SEPARATOR}{STDOUT}{MAIN_SEPARATOR}{f}")
        } else {
            format!("{OUTPUT}{MAIN_SEPARATOR}{STDERR}{MAIN_SEPARATOR}{f}")
        }
    }
    #[must_use]
    pub fn rust(&mut self) -> i32 {
        let verbose = self.args.contains(&"-v".to_string());
        let mut app = Builder::new();
        let mut builder = Builder::new();
        let mut clippy_table = Builder::new();
        let mut test_table = Builder::new();
        let mut audit_table = Builder::new();
        let mut check_table = Builder::new();
        let mut format_table = Builder::new();
        let prj = current_dir().expect("");
        let project = prj.to_str().expect("").split(MAIN_SEPARATOR).last();
        let description =
            read_to_string(File::open(".git/description").expect("failed to open file"))
                .expect("failed to parse file");
        app.push_record([project.expect(""), description.as_str()]);
        builder.push_record(["Task", "Status", "Datetime", "Help", "Output File"]);

        let audit = run("cargo", "audit", output("audit", true), true);

        if audit.eq(&0) {
            builder.push_record([
                "Audit",
                SUCCESS,
                self.date().as_str(),
                "",
                self.filename("audit", true).as_str(),
            ]);
        } else {
            builder.push_record([
                "Audit",
                FAILURE,
                self.date().as_str(),
                "cargo audit fix",
                self.filename("audit", true).as_str(),
            ]);
        }
        if verbose {
            audit_table.push_record(["Audit"]);
            let c = read_to_string(
                File::open(self.filename("audit", true).as_str()).expect("failed to open file"),
            )
            .expect("failed to parse file");
            audit_table.push_record([c.as_str()]);
            println!(
                "\n{}",
                audit_table
                    .build()
                    .with(Style::modern_rounded())
                    .to_string()
            );
        }
        let clippy = run(
            "cargo",
            "clippy --  -F keyword_idents
                        -F warnings
                        -F let-underscore 
                        -F rust-2018-compatibility
                        -F rust-2018-idioms 
                        -F rust-2021-compatibility
                        -F future-incompatible
                        -F unused
                        -F unused_crate_dependencies
                        -F unused_extern_crates
                        -F unused_macro_rules 
                        -F unused_results
                        -F unused_qualifications 
                        -F nonstandard-style
                        -F macro_use_extern_crate 
                        -F absolute_paths_not_starting_with_crate
                        -F ambiguous_glob_imports 
                        -F clippy::all
                        -F clippy::perf 
                        -F clippy::pedantic
                        -F clippy::style 
                        -F clippy::suspicious
                        -F clippy::correctness
                        -F clippy::nursery
                        -F clippy::complexity 
                        -D clippy::cargo",
            output("clippy", false),
            false,
        );
        if clippy.eq(&0) {
            builder.push_record([
                "Clippy",
                SUCCESS,
                self.date().as_str(),
                "",
                self.filename("clippy", false).as_str(),
            ]);
        } else {
            builder.push_record([
                "Clippy",
                FAILURE,
                self.date().as_str(),
                "cargo fix",
                self.filename("clippy", false).as_str(),
            ]);
        }
        if verbose {
            clippy_table.push_record(["Clippy"]);
            let c = read_to_string(
                File::open(self.filename("clippy", false).as_str()).expect("failed to open file"),
            )
            .expect("failed to parse file");
            clippy_table.push_record([c.as_str()]);
            println!(
                "\n{}",
                clippy_table
                    .build()
                    .with(Style::modern_rounded())
                    .to_string()
            );
        }
        let tests = run("cargo", "test  --no-fail-fast", output("test", true), true);
        if tests.eq(&0) {
            builder.push_record([
                "Test",
                SUCCESS,
                self.date().as_str(),
                "",
                self.filename("test", true).as_str(),
            ]);
        } else {
            builder.push_record([
                "Test",
                FAILURE,
                self.date().as_str(),
                "cargo test",
                self.filename("test", true).as_str(),
            ]);
        }
        if verbose {
            let c = read_to_string(
                File::open(self.filename("test", true).as_str()).expect("failed to open file"),
            )
            .expect("failed to parse file");
            test_table.push_record(["Test"]);
            test_table.push_record([c.as_str()]);
            println!(
                "\n{}",
                test_table.build().with(Style::modern_rounded()).to_string()
            );
        }
        let checkup = run("cargo", "check", output("check", false), false);

        if checkup.eq(&0) {
            builder.push_record([
                "Check",
                SUCCESS,
                self.date().as_str(),
                "",
                self.filename("check", false).as_str(),
            ]);
        } else {
            builder.push_record([
                "Check",
                FAILURE,
                self.date().as_str(),
                "cargo fix",
                self.filename("check", false).as_str(),
            ]);
        }
        if verbose {
            check_table.push_record(["Check"]);
            let c = read_to_string(
                File::open(self.filename("check", false).as_str()).expect("failed to open file"),
            )
            .expect("failed to parse file");
            check_table.push_record([c.as_str()]);
            println!(
                "\n{}",
                check_table
                    .build()
                    .with(Style::modern_rounded())
                    .to_string()
            );
        }

        let format = run(
            "cargo",
            "fmt -- --color never --check",
            output("format", true),
            true,
                    );

        if format.eq(&0) {
            builder.push_record([
                "Format",
                SUCCESS,
                self.date().as_str(),
                "",
                self.filename("format", true).as_str(),
            ]);
        } else {
            builder.push_record([
                "Format",
                FAILURE,
                self.date().as_str(),
                "cargo fmt",
                self.filename("format", true).as_str(),
            ]);
        }
        if verbose {
            format_table.push_record(["Format"]);
            let c = read_to_string(
                File::open(self.filename("format", true).as_str()).expect("failed to open file"),
            )
            .expect("failed to parse file");
            format_table.push_record([c.as_str()]);
            println!(
                "\n{}\n",
                format_table
                    .build()
                    .with(Style::modern_rounded())
                    .to_string()
            );
        }
        let table_status = builder.build().with(Style::modern_rounded()).to_string();
        println!("\n{table_status}\n");
        if format.eq(&0) && checkup.eq(&0) && tests.eq(&0) && clippy.eq(&0) && audit.eq(&0) {
            0
        } else {
            1
        }
    }
    #[must_use]
    pub fn run(&mut self, l: &Language) -> i32 {
        match l {
            Rust => self.rust(),
            Go => self.go(),
            Docker => self.docker(),
            Make => self.make(),
            Composer => self.composer(),
            Cmake => self.cmake(),
            Unknown => 1,
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len().eq(&2) && args.get(1).unwrap().eq("init") {
        init();
        okay("Your project it's now tracked by zuu");
        exit(0);
    }
    if args.contains(&"--watch".to_string()) {
        watch(s);
    }
    let code = check(detect(), s);
    status();
    print!("{}", ansi_escapes::CursorShow);
    exit(code);
}
