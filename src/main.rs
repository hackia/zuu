use core::str;
use std::io::Write;
use std::{
    fs::{read_to_string, File},
    io::Error,
    path::Path,
    process::Command,
};

#[cfg(feature = "cli")]
use std::{thread::sleep, time::Duration};
const ZUU_STDERR_FILE: &str = "/tmp/zuu-stderr";
const ZUU_STDOUT_FILE: &str = "/tmp/zuu-stdout";

pub mod bash;
pub mod c;
pub mod clojure;
pub mod cobol;
pub mod crystal;
pub mod dart;
pub mod elixir;
pub mod erlang;
pub mod f_sharp;
pub mod fortran;
pub mod go;
pub mod groovy;
pub mod haskell;
pub mod java;
pub mod julia;
pub mod kotlin;
pub mod lua;
pub mod matlab;
pub mod nim;
pub mod objective_c;
pub mod perl;
pub mod php;
pub mod r;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod swift;
pub mod typescript;
pub mod utils;
pub mod vlang;

#[cfg(feature = "d")]
pub mod d;
pub mod ocaml;
pub mod python;
pub mod tcl;
pub mod vhdl;

#[cfg(feature = "ui")]
use crossterm::event::{self, Event, KeyCode};
#[cfg(feature = "ui")]
use ratatui::{
    init,
    layout::Alignment,
    prelude::CrosstermBackend,
    restore,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use toml::Value;

#[cfg(feature = "cli")]
use indicatif::ProgressBar;
#[cfg(feature = "ui")]
use std::io::Stdout;

fn shell(filename: &str) -> bool {
    if let Ok(child) = Command::new(filename)
        .stdout(File::create(ZUU_STDOUT_FILE).expect("failed to create file"))
        .stderr(File::create(ZUU_STDERR_FILE).expect("failed to create file"))
        .current_dir(".")
        .output()
    {
        return child.status.success();
    }
    false
}

#[cfg(feature = "cli")]
fn zuu_cli() -> Result<(), Error> {
    #[cfg(target_os = "linux")]
    assert!(Command::new("clear").spawn().is_ok());
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    let before_cargo = values.get("before-cargo");
    let after_cargo = values.get("after-cargo");
    let cargo = values.get("cargo");
    if let Some(data) = before_cargo {
        let hooks = data.as_array().expect("msg");
        let pb = ProgressBar::new_spinner().with_message("run pre cargo hooks");
        pb.enable_steady_tick(Duration::from_millis(100));
        for hook in hooks {
            if let Some(h) = hook.as_str() {
                if shell_exec(h) {
                    pb.inc(1);
                    sleep(Duration::from_secs(1));
                } else {
                    return error();
                }
            }
        }
        pb.finish_and_clear();
    }
    if let Some(data) = cargo {
        let hooks = data.as_array().expect("msg");
        let pb = ProgressBar::new_spinner().with_message("run cargo hooks");
        pb.enable_steady_tick(Duration::from_millis(100));
        for hook in hooks {
            if let Some(h) = hook.as_str() {
                if shell_exec(h) {
                    pb.inc(1);
                    sleep(Duration::from_secs(1));
                } else {
                    return error();
                };
            }
        }
        pb.finish_and_clear();
    }
    if let Some(data) = after_cargo {
        let hooks = data.as_array().expect("msg");
        let pb = ProgressBar::new(hooks.len() as u64).with_message("run post cargo hooks");
        pb.enable_steady_tick(Duration::from_millis(100));
        for hook in hooks {
            if let Some(h) = hook.as_str() {
                if shell_exec(h) {
                    pb.inc(1);
                    sleep(Duration::from_secs(1));
                } else {
                    return error();
                }
            }
        }
        pb.finish_and_clear();
    }
    if let Some(data) = after_cargo {
        let hooks = data.as_array().expect("msg");
        let pb = ProgressBar::new(hooks.len() as u64).with_message("run post cargo hooks");
        pb.enable_steady_tick(Duration::from_millis(100));
        for hook in hooks {
            if let Some(h) = hook.as_str() {
                if shell_exec(h) {
                    pb.inc(1);
                    sleep(Duration::from_secs(1));
                } else {
                    return error();
                }
            }
        }
        pb.finish_and_clear();
    }
    assert!(gen_badges(true).is_ok());
    Ok(())
}

fn generate_zuu() -> Result<(), Error> {
    use std::{fs::remove_file, path::Path};

    if Path::new("zuu.toml").exists() {
        remove_file("zuu.toml")?;
    }
    let mut zuu: File = File::create_new("zuu.toml")?;

    assert!(write!(
        zuu,
        "hooks = []\n\n[badge]\nsuccess = [\"curl https://img.shields.io/badge/zuu-success-green -o zuu.svg\"]\nfailure = [\"curl https://img.shields.io/badge/zuu-failure-red -o zuu.svg\"]").is_ok());
    Ok(())
}
#[cfg(feature = "ui")]
fn run_zuu() -> Result<(), Error> {
    let zuu: String = read_to_string("zuu.toml").unwrap_or_default();
    let values: Value = zuu.parse::<Value>().unwrap_or(Value::String(String::new()));
    let hooks = values.get("hooks");
    if let Some(data) = hooks {
        let hooks = data.as_array().expect("msg");
        for hook in hooks {
            if let Some(h) = hook.as_str() {
                if shell(h) {
                    continue;
                }
                return error();
            }
        }
    }
    Ok(())
}
#[cfg(feature = "ui")]
fn zuu_ui() {
    let mut term: Terminal<CrosstermBackend<Stdout>> = init();
    term.clear().expect("failed to clear screen");
    term.set_cursor_position((0, 0))
        .expect("failed to restore pos of cursor");
    assert!(term
        .draw(|f| {
            f.render_widget(
                Paragraph::new(
                    "F2     ==> Check source code\nEsc    ==> Exit\nArrows ==> Navigate in errors",
                )
                .block(
                    Block::default()
                        .title(" Zuu ")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::all()),
                ),
                f.area(),
            );
        })
        .is_ok());
    let mut success: Result<(), Error>;
    let mut v: u16 = 0;
    let mut h: u16 = 0;
    loop {
        if let Event::Key(key) = event::read().expect("msg") {
            if key.code == KeyCode::Esc {
                break;
            } else if key.code == KeyCode::F(2) {
                assert!(term
                    .draw(|f| {
                        f.render_widget(
                            Paragraph::new("Checking source code...").block(
                                Block::default()
                                    .title(" Zuu ")
                                    .title_alignment(Alignment::Center)
                                    .borders(Borders::all()),
                            ),
                            f.area(),
                        );
                    })
                    .is_ok());
                success = run_zuu();
                if success.is_err() {
                    loop {
                        assert!(term
                            .draw(|f| {
                                f.render_widget(
                                    Paragraph::new(read_to_string(ZUU_STDERR_FILE).expect(""))
                                        .block(
                                            Block::default()
                                                .borders(Borders::all())
                                                .title(" Zuu ")
                                                .title_alignment(Alignment::Center),
                                        )
                                        .scroll((v, h)),
                                    f.area(),
                                );
                            })
                            .is_ok());

                        if let Event::Key(key) = event::read().expect("msg") {
                            if key.code == KeyCode::Down {
                                v += 1;
                            } else if key.code == KeyCode::Up {
                                if v.gt(&0) {
                                    v -= 1;
                                }
                            } else if key.code == KeyCode::Left {
                                if h.gt(&0) {
                                    h -= 1;
                                }
                            } else if key.code == KeyCode::Right {
                                h += 1;
                            } else if key.code == KeyCode::Esc {
                                break;
                            }
                        }
                    }
                } else {
                    assert!(term
                        .draw(|f| {
                            f.render_widget(
                                Paragraph::new("code can be commited").block(
                                    Block::default()
                                        .title(" Zuu ")
                                        .title_alignment(Alignment::Center)
                                        .borders(Borders::all()),
                                ),
                                f.area(),
                            );
                        })
                        .is_ok());
                }
            }
        }
    }

    restore();
}

fn error() -> Result<(), Error> {
    Err(Error::other(
        read_to_string(ZUU_STDERR_FILE)
            .expect("No founded error file")
            .as_str(),
    ))
}

fn main() {
    if Path::new("zuu.toml").exists().eq(&false) {
        assert!(generate_zuu().is_ok());
    }
    #[cfg(feature = "ui")]
    zuu_ui();

    #[cfg(feature = "cli")]
    return match zuu_cli() {
        Ok(()) => println!("Code can be commited"),
        Err(e) => {
            eprintln!("{e}");
        }
    };
}
