use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs::read_to_string;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use toml::Value;

pub struct Bar {
    pub bar: MultiProgress,
    pub template: String,
    pub style: ProgressStyle,
    pub progress: String,
    pub tick: Duration,
}
pub struct Zuu {
    pub options: Options,
    pub badges: Badges,
    pub hooks: Hooks,
    pub groups: Groups,
    pub bar: Bar,
}
#[derive(Copy, Clone)]
pub struct Options {
    pub verify_project: bool,
    pub format: bool,
    pub install: bool,
    pub clean: bool,
    pub doc: bool,
    pub audit: bool,
    pub test: bool,
    pub check: bool,
    pub update: bool,
    pub deny: bool,
    pub outdated: bool,
    pub watch: bool,
}
pub struct Badges {
    pub success: String,
    pub failure: String,
}

#[derive(Debug)]
pub struct Hooks {
    pub before_all: Vec<String>,
    pub before_each: Vec<String>,
    pub after_all: Vec<String>,
    pub for_each: Vec<String>,
    pub after_each: Vec<String>,
}

pub struct Groups {
    pub allow: Vec<String>,
    pub warn: Vec<String>,
    pub forbid: Vec<String>,
}

fn groups() -> Groups {
    let mut configuration: Groups = Groups {
        allow: Vec::new(),
        warn: Vec::new(),
        forbid: Vec::new(),
    };
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(groups) = values.get("groups") {
        if let Some(groups) = groups.as_table() {
            if let Some(allow) = groups.get("allow") {
                if let Some(allow) = allow.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for a in allow {
                        x.push(a.to_string());
                    }
                    configuration.allow = x;
                }
            }
            if let Some(warn) = groups.get("warn") {
                if let Some(warn) = warn.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for a in warn {
                        x.push(a.to_string());
                    }
                    configuration.warn = x;
                }
            }
            if let Some(forbid) = groups.get("forbid") {
                if let Some(forbid) = forbid.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for a in forbid {
                        x.push(a.to_string());
                    }
                    configuration.forbid = x;
                }
            }
        }
    }
    configuration
}
fn hooks() -> Hooks {
    let mut configuration: Hooks = Hooks {
        before_all: Vec::new(),
        before_each: Vec::new(),
        after_all: Vec::new(),
        for_each: Vec::new(),
        after_each: Vec::new(),
    };
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(options) = values.get("hooks") {
        if let Some(options) = options.as_table() {
            if let Some(before_all) = options.get("before-all") {
                if let Some(before_all) = before_all.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for before in before_all {
                        if let Some(b) = before.as_table() {
                            if let Some(c) = b.get("command")
                            {
                                dbg!(c);
                            }
                            if let Some(command) = b.get("before-run") {
                                if let Some(command) = command.as_str() {
                                    dbg!(command);
                                    x.push(command.to_string());
                                }
                            }
                        }
                    }
                    configuration.before_all = x;
                }
            }
            if let Some(after_all) = options.get("after-all") {
                if let Some(after_all) = after_all.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for after in after_all {
                        if let Some(a) = after.as_str() {
                            x.push(a.to_string());
                        }
                    }
                    configuration.after_all = x;
                }
            }
            if let Some(before_each) = options.get("before-each") {
                if let Some(before_each) = before_each.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for before in before_each {
                        if let Some(b) = before.as_str() {
                            x.push(b.to_string());
                        }
                    }
                    configuration.before_each = x;
                }
            }
            if let Some(for_each) = options.get("for-each") {
                if let Some(for_each) = for_each.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for each in for_each {
                        if let Some(each) = each.as_str() {
                            x.push(each.to_string());
                        }
                    }
                    configuration.for_each = x;
                }
            }
            if let Some(after_each) = options.get("after-each") {
                if let Some(after_each) = after_each.as_array() {
                    let mut x: Vec<String> = Vec::new();
                    for each in after_each {
                        if let Some(each) = each.as_str() {
                            x.push(each.to_string());
                        }
                    }
                    configuration.after_each = x;
                }
            }
        }
    }
    configuration
}
fn options() -> Options {
    let mut configuration: Options = Options {
        verify_project: false,
        format: false,
        install: false,
        clean: false,
        doc: false,
        audit: false,
        test: false,
        check: false,
        update: false,
        deny: false,
        outdated: false,
        watch: false,
    };
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(options) = values.get("options") {
        if let Some(options) = options.as_table() {
            if let Some(format) = options.get("format") {
                if let Some(format) = format.as_bool() {
                    configuration.format = format;
                }
            }
            if let Some(verify_project) = options.get("verify-project") {
                if let Some(verify_project) = verify_project.as_bool() {
                    configuration.verify_project = verify_project;
                }
            }
            if let Some(install) = options.get("install") {
                if let Some(install) = install.as_bool() {
                    configuration.install = install;
                }
            }
            if let Some(clean) = options.get("clean") {
                if let Some(clean) = clean.as_bool() {
                    configuration.clean = clean;
                }
            }
            if let Some(doc) = options.get("doc") {
                if let Some(doc) = doc.as_bool() {
                    configuration.doc = doc;
                }
            }
            if let Some(audit) = options.get("audit") {
                if let Some(audit) = audit.as_bool() {
                    configuration.audit = audit;
                }
            }
            if let Some(test) = options.get("test") {
                if let Some(test) = test.as_bool() {
                    configuration.test = test;
                }
            }
            if let Some(check) = options.get("check") {
                if let Some(check) = check.as_bool() {
                    configuration.check = check;
                }
            }
            if let Some(update) = options.get("update") {
                if let Some(update) = update.as_bool() {
                    configuration.update = update;
                }
            }
            if let Some(deny) = options.get("deny") {
                if let Some(deny) = deny.as_bool() {
                    configuration.deny = deny;
                }
            }
            if let Some(outdated) = options.get("outdated") {
                if let Some(outdated) = outdated.as_bool() {
                    configuration.outdated = outdated;
                }
            }
            if let Some(watch) = options.get("watch") {
                if let Some(watch) = watch.as_bool() {
                    configuration.watch = watch;
                }
            }
        }
    }
    configuration
}

fn badges() -> Badges {
    let mut configuration: Badges = Badges {
        success: "".to_string(),
        failure: "".to_string(),
    };
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(config) = values.get("badges") {
        if let Some(config) = config.as_table() {
            if let Some(success) = config.get("success") {
                if let Some(success) = success.as_str() {
                    configuration.success = success.to_string();
                }
            }
            if let Some(failure) = config.get("failure") {
                if let Some(failure) = failure.as_str() {
                    configuration.failure = failure.to_string();
                }
            }
        }
    }
    configuration
}
fn bar_template() -> String {
    let x = String::new();
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(config) = values.get("bar") {
        if let Some(template) = config.get("template") {
            if let Some(template) = template.as_str() {
                return template.to_string();
            }
        }
    }
    x
}
fn bar_progress() -> String {
    let x = String::new();
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    if let Some(config) = values.get("bar") {
        if let Some(progress) = config.get("progress") {
            if let Some(progress) = progress.as_str() {
                return progress.to_string();
            }
        }
    }
    x
}
fn bar_style() -> ProgressStyle {
    if let Ok(x) = ProgressStyle::with_template(bar_template().as_str()) {
        return x;
    }
    ProgressStyle::default_spinner()
}
fn zuu() -> Zuu {
    Zuu {
        options: options(),
        badges: badges(),
        hooks: hooks(),
        groups: groups(),
        bar: Bar {
            bar: MultiProgress::new(),
            template: bar_template(),
            style: bar_style(),
            progress: bar_progress(),
            tick: Duration::from_millis(100),
        },
    }
}

fn exec(verb: String, args: &[&str]) -> bool {
    if let Ok(mut x) = Command::new("cargo")
        .arg(verb)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(".")
        .spawn()
    {
        if let Ok(status) = x.wait() {
            return status.success().eq(&true);
        }
        return false;
    }
    false
}
fn shell_exec(c: &str) -> bool {
    if let Ok(mut child) = Command::new("sh")
        .arg("-c")
        .arg(c)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .current_dir(".")
        .spawn()
    {
        if let Ok(s) = child.wait() {
            return s.success();
        }
    }
    false
}

fn bar(
    tick: Duration,
    style: &ProgressStyle,
    len: u64,
    message: String,
    end_message: String,
) -> ProgressBar {
    let pb: ProgressBar = ProgressBar::new(len);
    pb.enable_steady_tick(tick);
    pb.set_style(style.clone());
    pb.set_message(message);
    pb.finish_with_message(end_message);
    pb
}

fn run_hook(verb: &str, x: &Vec<String>, o: &MultiProgress) {
    for (index, hook) in x.iter().enumerate() {
        if shell_exec(hook.as_str()) {
            assert!(o
                .println(format!(
                    "{verb} hook n°{index} has been executed successfully"
                ))
                .is_ok());
        } else {
            assert!(o
                .println(format!(
                    "{verb} hook n°{index} failed to execute the hook {hook}"
                ))
                .is_ok());
        }
    }
}
fn main() -> std::io::Result<()> {
    dbg!(hooks());
    let app: Zuu = zuu();
    let output = app.bar.bar;
    let mut threads = vec![];
    run_hook("before all", &app.hooks.before_all, &output);
    for _i in 0..=10 {
        threads.push(thread::spawn(move || {}));
    }
    for thread in threads {
        let _ = thread.join();
    }
    run_hook("after all", &app.hooks.after_all, &output);
    Ok(())
}
