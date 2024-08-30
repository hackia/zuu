use std::{
    env::args,
    fs::{self, File},
    io::Error,
    io::Write,
    path::Path,
    process::Command,
};
use toml::Value;

fn shell_exec(c: &str) {
    let x: Vec<&str> = c.split_whitespace().collect();
    if let Ok(mut child) = Command::new("sh")
        .args(["-c", x.join(" ").as_str()])
        .current_dir(".")
        .spawn()
    {
        if let Ok(s) = child.wait() {
            assert!(s.success());
        }
    }
}

fn run(c: &str) {
    if let Ok(mut child) = Command::new("cargo")
        .args(c.split_whitespace())
        .current_dir(".")
        .spawn()
    {
        if let Ok(s) = child.wait() {
            assert!(s.success());
        }
    }
}

fn parse_shell(value: &Value) {
    if let Some(data) = value.as_array() {
        for hook in data {
            if let Some(h) = hook.as_str() {
                shell_exec(h);
            }
        }
    }
}
fn parse_cargo(value: &Value) {
    if let Some(data) = value.as_array() {
        for hook in data {
            if let Some(h) = hook.as_str() {
                run(h);
            }
        }
    }
}
fn run_zuu() -> Result<(), Error> {
    let mut clippy: String = String::from("clippy --");

    let zuu: String = fs::read_to_string("zuu.toml").unwrap_or_default();

    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));

    let before_cargo = values.get("before-cargo");

    let after_cargo = values.get("after-cargo");

    let cargo = values.get("cargo");

    if let Some(a) = before_cargo {
        parse_shell(a);
    }
    if let Some(a) = cargo {
        parse_cargo(a);
    }
    if let Some(allowed) = values.get("allow") {
        if let Some(data) = allowed.as_array() {
            for warn in data {
                clippy.push_str(
                    format!(" -A clippy::{} ", warn.as_str().unwrap_or_default()).as_str(),
                );
            }
        }
    }
    if let Some(warning) = values.get("warn") {
        if let Some(data) = warning.as_array() {
            for warn in data {
                clippy.push_str(
                    format!(" -W clippy::{} ", warn.as_str().unwrap_or_default()).as_str(),
                );
            }
        }
    }
    if let Some(forbiden) = values.get("forbid") {
        if let Some(data) = forbiden.as_array() {
            for forbid in data {
                clippy.push_str(
                    format!(" -F clippy::{} ", forbid.as_str().unwrap_or_default()).as_str(),
                );
            }
        }
    }
    if let Ok(mut child) = Command::new("cargo")
        .args(clippy.split_whitespace())
        .current_dir(".")
        .spawn()
    {
        if let Ok(code) = child.wait() {
            if code.success() {
                println!("\x1b[1;32m    Finished\x1b[0;37m Code can be commited\x1b[0m");
                if let Some(c) = after_cargo {
                    parse_shell(c);
                }
                return Ok(());
            }
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Source code not valid",
            ));
        }
    }
    Err(Error::last_os_error())
}
fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.is_empty() && Path::new("zuu.toml").exists() {
        return run_zuu();
    } else if args.len() == 2 && args.get(1).unwrap_or(&String::new()).eq("init") {
        let mut zuu: File = File::create_new("zuu.toml")?;
        return write!(
            zuu,
            "allow = [\"cargo\"]\nwarn = []\nforbid = [\n\t\"nursery\",\n\t\"perf\",\n\t\"complexity\",\n\t\"style\",\n\t\"pedantic\",\n\t\"suspicious\",\n\t\"correctness\"\n]\nbefore-cargo = []\ncargo = [\n\t\"verify-project\",\n\t\"check --all-targets --profile=test\",\n\t\"audit\",\n\t\"test -j 4 --no-fail-fast -- --show-output\",\n\t\"fmt --check\",\n\t\"outdated\"\n]\nafter-cargo = []\n"
        );
    }

    Ok(())
}
