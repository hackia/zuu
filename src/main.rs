use chrono::Utc;
use colored_truecolor::Colorize;
use git2::{Diff, DiffFormat, DiffOptions, ErrorClass, ErrorCode, Index, Repository, Status};
use inquire::{Confirm, MultiSelect, Select, Text};
use std::fs::{read_to_string, remove_file};
use std::io::ErrorKind;
use std::{env::args, fs::File, io::Error, io::Write, path::Path, process::Command};
use toml::Value;
const CLIPPY_GROUPS: [&str; 8] = [
    "cargo",
    "complexity",
    "style",
    "nursery",
    "pedantic",
    "suspicious",
    "correctness",
    "perf",
];

const HOOKS: [&str; 7] = [
    "verify-project",
    "check --all-targets --profile=test",
    "deny check",
    "audit",
    "test -j 4 --no-fail-fast -- --show-output",
    "fmt --check",
    "outdated",
];

const CHECK_FILE: &str = "zen";

const LANG: &str = "en_US";

const COMMITS_TYPES: [&str; 68] = [
    "Star: New feature or enhancement",
    "Comet: Bug fix or error resolution",
    "Nebula: Code refactoring",
    "Pulsar: Performance improvement",
    "Quasar: Documentation or clarity improvement",
    "Asteroid Belt: Code cleanup and maintenance",
    "Solar Flare: Testing-related changes",
    "Dwarf Planet: Minor updates or fixes",
    "Terraform: Infrastructure changes",
    "Black Hole: Removing large chunks of code or features",
    "Wormhole: Merging branches or connecting code parts",
    "Big Bang: Initial commit or major feature start",
    "Launch: Deploying to production or releasing a version",
    "Lightspeed: Significant performance improvements",
    "Mission Control: Project management changes",
    "Spacewalk: Urgent hotfixes",
    "Moon Landing: Major milestone or goal completion",
    "First Contact: Initial integrations with external systems",
    "Interstellar Communication: Improving documentation or communication",
    "Solar Eclipse: Temporarily masking functionality",
    "Supernova: Major, transformative change",
    "Meteor Shower: Series of small changes or fixes",
    "Solar Wind: Refactoring code structure",
    "Lunar Eclipse: Temporarily disabling a feature",
    "Cosmic Dawn: Initial implementation of a feature",
    "Solar Storm: Rapid, impactful changes",
    "Lunar Transit: Minor, temporary change",
    "Perihelion: Brings the project closer to its goals or objectives",
    "Aphelion: Immediate goals, but is necessary for long-term progress",
    "White Dwarf: Improving code comments or documentation",
    "Red Giant: Expanding a feature or functionality",
    "Neutron Star: Optimizing code for performance",
    "Binary Star: Merging features or components",
    "Brown Dwarf: Undeveloped feature with potential",
    "Quark Star: Experimental or speculative change",
    "Rogue Planet: Independent change",
    "Stellar Nursery: Creation of new components",
    "Planetary Nebula: Removal or deprecation of a component",
    "Globular Cluster: Collection of related changes",
    "Void: Removal of a module, component, or feature",
    "Gravity: Resolving merge conflicts or dependencies",
    "Dark Matter: Fixing unknown or mysterious bugs",
    "Time Dilation: Improving code performance",
    "Spacetime: Changes to date, time, or scheduling",
    "Gravitational Lensing: Altering data or information flow",
    "Cosmic String: Connecting code parts",
    "Quantum Fluctuation: Small, random change",
    "Hawking Radiation: Removing technical debt",
    "Quantum Entanglement: Establishing close relationships between code parts",
    "Gravitational Redshift: Slowing down or reducing code performance",
    "Space Probe: Testing new features or technologies",
    "Station: Creating or improving environments",
    "Rocket Launch: Deploying to production",
    "Spacewalk: Urgent production hotfixes",
    "Space Elevator: Making codebase more accessible",
    "Warp Drive: Significant speed improvement",
    "Dyson Sphere: Comprehensive optimization of a specific area",
    "Generation Ship: Long-term project for a self -sustaining system",
    "Lagrange Point: Stabilizing or balancing code parts",
    "Orbital Maneuver: Changing project direction",
    "Mission Control: Represents project management-related changes",
    "Moon Landing: Celebrates the completion of major milestones",
    "Interstellar Travel: Migration to a new architecture or language",
    "Rover: Exploration of new technologies or approaches",
    "Singularity: Resolution of a complex or hard-to-reproduce issue",
    "Relativity: Changes related to time, dates, or timestamps",
    "Expansion: Scaling up the system or increasing capacity",
    "Big Crunch: Reduction of codebase size or removal of features",
];
const COMMIT_TEMPLATE: &str = "%type%(%scope%): %summary%\n\n\tThe following changes were made :\n\n%why%\n\n%footer%\n\n\tAuthored by :\n\n\t\t* %author% <%email%> the %date%\n";

fn check_commit(sentence: &str) -> Result<(), Error> {
    if let Ok(mut f) = File::create(CHECK_FILE) {
        assert!(writeln!(f, "{sentence}").is_ok());
        if let Ok(child) = Command::new("hunspell")
            .arg("-d")
            .arg(LANG)
            .arg("-l")
            .arg(CHECK_FILE)
            .output()
        {
            if child.stdout.is_empty() {
                return Ok(());
            }
        }
        return arrange_commit();
    }
    Err(Error::last_os_error())
}
fn print_diff(diff: &Diff<'_>) -> Result<(), git2::Error> {
    if let Ok(stats) = diff.stats() {
        let x = diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            let origin = line.origin();
            let content: String = String::from_utf8_lossy(line.content()).into_owned();
            match origin {
                '-' => print!("{} {}", "-".red(), content.red()),
                '+' => print!("{} {}", "+".green(), content.green()),
                '@' => print!("  {}", content.cyan()),
                _ => print!("  {content}"),
            }
            true
        });

        print!(
            "\n  {} files changed, {} insertions(+), {} deletion(-)\n",
            stats.files_changed(),
            stats.insertions(),
            stats.deletions(),
        );
        return x;
    }
    Err(git2::Error::new(
        ErrorCode::Ambiguous,
        ErrorClass::Repository,
        "no stats",
    ))
}
fn diff(path: &str) -> Result<(), Error> {
    if let Ok(r) = Repository::open(path) {
        let mut opts: DiffOptions = DiffOptions::new();
        if let Ok(changes) = r.diff_index_to_workdir(
            None,
            Some(&mut opts.include_untracked(true).recurse_untracked_dirs(true)),
        ) {
            assert!(print_diff(&changes).is_ok());
            return Ok(());
        }
    }
    Err(Error::last_os_error())
}

fn add(path: &str) -> Option<Index> {
    if let Ok(r) = Repository::open(path) {
        return r.statuses(None).map_or_else(
            |_| None,
            |status| {
                let mut file_options: Vec<String> = Vec::new();
                for entry in &status {
                    let status: Status = entry.status();
                    if status.is_wt_new() || status.is_wt_modified() {
                        let path = entry.path().unwrap_or_default();
                        file_options.push(path.to_string());
                    }
                }
                if file_options.is_empty() {
                    println!("No files to add.");
                    None
                } else if let Ok(selected_files) =
                    MultiSelect::new("Select files to add:", file_options).prompt()
                {
                    let mut index = r.index().expect("msg");
                    for file in &selected_files {
                        index.add_path(file.as_ref()).expect("msg");
                    }
                    index.write().expect("msg");
                    println!("Added {} files to the index.", selected_files.len());
                    Some(index)
                } else {
                    None
                }
            },
        );
    }
    None
}

fn msg(m: &str, r: &str) -> Result<(), Error> {
    if let Ok(mut child) = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(m)
        .current_dir(r)
        .spawn()
    {
        if let Ok(code) = child.wait() {
            if code.success() {
                return Ok(());
            }
        }
    }
    Err(Error::last_os_error())
}
fn commit(path: &str) -> Result<(), Error> {
    assert!(diff(path).is_ok());
    let index = add(path);
    if index.is_none() {
        return Err(Error::new(ErrorKind::NotFound, "No changes"));
    }
    msg(
        COMMIT_TEMPLATE
            .replace("%type%", get_commit_types().as_str())
            .replace("%scope%", get_scope().as_str())
            .replace("%summary%", get_summary().as_str())
            .replace("%why%", get_why().as_str())
            .replace("%footer%", get_footer().as_str())
            .replace("%date%", Utc::now().date_naive().to_string().as_str())
            .replace("%author%", name().as_str())
            .replace("%email%", email().as_str())
            .as_str(),
        path,
    )
}

fn commit_types_with_help() -> [&'static str; 68] {
    let mut x = COMMITS_TYPES;
    x.sort_unstable();
    x
}

fn commit_scope() -> String {
    let mut scope: String;
    loop {
        scope = Text::new("Please enter the commit scope : ")
            .prompt()
            .unwrap_or_default();
        if scope.is_empty() {
            continue;
        }
        if scope.len().gt(&20) {
            println!("scope can be superior to 20 character");
            continue;
        }
        if confirm(
            format!("Really use the commit scope : {scope}").as_str(),
            false,
        )
        .eq(&true)
        {
            break;
        }
    }
    scope
}

fn get_commit_types() -> String {
    let mut t: String;
    loop {
        t = Select::new(
            "Please enter the commit type : ",
            commit_types_with_help().to_vec(),
        )
        .prompt()
        .unwrap_or_default()
        .to_string();
        if t.is_empty() {
            continue;
        }
        if confirm(format!("Really use the commit type : {t}").as_str(), false) {
            break;
        }
    }
    let x: Vec<&str> = t.split(':').collect();
    let mut s: String = String::from("\n");
    s.push_str((*x.first().unwrap()).to_string().as_str());
    s
}

fn commit_summary() -> String {
    let mut summary: String;
    loop {
        summary = Text::new("Please enter the commit summary : ")
            .prompt()
            .unwrap_or_default();
        if summary.is_empty() {
            continue;
        }
        if summary.len().gt(&50) {
            println!("Summary must be contains less than 50 characters");
            continue;
        }
        if confirm(format!("Use the summary : {summary}").as_str(), false) {
            break;
        }
    }
    summary
}

fn commit_why() -> String {
    let mut why: String = String::new();
    loop {
        let w = Text::new("Please explain the reasoning behind the change : ")
            .prompt()
            .unwrap_or_default();
        if w.is_empty() {
            continue;
        }
        if w.len().gt(&50) {
            println!("The reasoning behind the change must be contains less than 50 characters");
            continue;
        }
        why.push_str(format!("\n\t\t* {w}").as_str());
        if confirm("Continue to write the changes : ", false) {
            continue;
        }
        break;
    }
    why
}
fn commit_footer() -> String {
    let mut footer: String = String::new();
    if confirm("Code has breaking changes ?", false) {
        footer.push_str("\n\tThe following changes break :\n");
        loop {
            let b = Text::new("Please enter the breaking change description: ")
                .prompt()
                .unwrap_or_default();
            if b.is_empty() {
                continue;
            }
            if confirm(
                format!("Use breaking change description : {b}").as_str(),
                false,
            ) {
                footer.push_str(format!("\n\t\t* {b}\n").as_str());
                if confirm("Add a new description line ?", false).eq(&true) {
                    continue;
                }
                break;
            }
        }
    }
    if confirm("Code has resolving issues ?", false) {
        footer.push_str("\n\tThe commit resolve their issues :\n");
        loop {
            footer.push_str("\n\t\tFixes ");
            loop {
                let f = Text::new("Please enter the issue number : ")
                    .prompt()
                    .unwrap_or_default();
                if f.is_empty() {
                    continue;
                }
                footer.push_str(format!("#{f}\n").as_str());
                break;
            }
            if confirm("Code resolving an other issues ?", false) {
                continue;
            }
            break;
        }
    }
    if confirm("Code close an issue ?", false) {
        footer.push_str("\n\tThe commit close their issues :\n");
        loop {
            footer.push_str("\n\t\tCloses ");
            loop {
                let f = Text::new("Please enter the issue number : ")
                    .prompt()
                    .unwrap_or_default();
                if f.is_empty() {
                    continue;
                }
                footer.push_str(format!("#{f}\n").as_str());
                break;
            }
            if confirm("Code resolve an other issue ?", false) {
                continue;
            }
            break;
        }
    }
    footer
}

fn get_scope() -> String {
    let mut scope: String;
    loop {
        scope = commit_scope();
        if check_commit(scope.as_str()).is_ok() {
            break;
        }
    }
    scope
}

fn get_summary() -> String {
    let mut summary: String;
    loop {
        summary = commit_summary();
        if check_commit(summary.as_str()).is_ok() {
            break;
        }
    }
    summary
}

fn get_why() -> String {
    let mut why: String;
    loop {
        why = commit_why();
        if check_commit(why.as_str()).is_ok() {
            break;
        }
    }
    why
}
fn get_footer() -> String {
    let mut footer: String;
    loop {
        footer = commit_footer();
        if check_commit(footer.as_str()).is_ok() {
            break;
        }
    }
    footer
}

fn confirm(msg: &str, default: bool) -> bool {
    if let Ok(r) = Confirm::new(msg).with_default(default).prompt() {
        return r;
    }
    false
}

fn email() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.email")
            .current_dir(".")
            .output()
            .expect("git email not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

fn name() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .current_dir(".")
            .output()
            .expect("username not found")
            .stdout,
    )
    .expect("msg")
    .trim()
    .to_string()
}

fn arrange_commit() -> Result<(), Error> {
    if let Ok(mut child) = Command::new("hunspell")
        .arg("-d")
        .arg(LANG)
        .arg(CHECK_FILE)
        .spawn()
    {
        if let Ok(code) = child.wait() {
            return if code.success() {
                Ok(())
            } else {
                let content = read_to_string(CHECK_FILE).unwrap_or_default();
                check_commit(content.as_str())
            };
        }
    }
    Err(Error::last_os_error())
}

fn decrease(g: &mut Vec<String>, data: &[String]) {
    for d in data {
        g.retain(|x| !x.eq(d));
    }
}
fn generate_zuu() -> Result<(), Error> {
    if Path::new("zuu.toml").exists() {
        remove_file("zuu.toml")?;
    }
    let mut zuu: File = File::create_new("zuu.toml")?;

    let mut groups: Vec<String> = CLIPPY_GROUPS.map(String::from).to_vec();
    let allowed = MultiSelect::new("Select the allowed groups : ", groups.clone())
        .prompt()
        .unwrap_or_else(|_| Vec::from(["cargo".to_string(), "pedantic".to_string()]));

    decrease(&mut groups, &allowed.clone());

    let warn = MultiSelect::new("Select the warning groups : ", groups.clone())
        .prompt()
        .unwrap_or_else(|_| groups.clone());

    decrease(&mut groups, &warn.clone());

    assert!(write!(
        zuu,
        "allow = {allowed:?}\nwarn = {warn:?}\nforbid = {groups:?}\nbefore-cargo = []\ncargo = {HOOKS:?}\nafter-cargo = []"
    )
        .is_ok());
    Ok(())
}

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
fn run_zuu(args: &[String]) -> Result<(), Error> {
    let mut clippy: String = String::from("clippy -- -W warnings");

    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();

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
    if let Some(forbidden) = values.get("forbid") {
        if let Some(data) = forbidden.as_array() {
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
                if let Some(c) = args.get(1) {
                    if c.eq("commit") {
                        assert!(commit(".").is_ok());
                        return Ok(());
                    }
                }
                return Ok(());
            }
            return Err(Error::new(ErrorKind::InvalidData, "Source code not valid"));
        }
        return Err(Error::new(ErrorKind::InvalidData, "Source code not valid"));
    }
    Err(Error::last_os_error())
}
fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if Path::new("zuu.toml").exists() {
        run_zuu(&args)
    } else if args.len() == 2 && args.get(1).unwrap_or(&String::new()).eq("init") {
        generate_zuu()
    } else {
        Err(Error::new(
            ErrorKind::Unsupported,
            "argument not recognized",
        ))
    }
}
