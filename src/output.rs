use crossterm::terminal::{Clear, ClearType};
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::size,
};
use std::{
    io::{stdout, Error},
    process::{Command, ExitCode},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};

#[doc = "The waiting task spinner strings"]
pub const SPINNERS: [&str; 4] = [". ", "..", ".:", "::"];

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
///
///
pub fn ok(description: &str, x: usize) {
    if let Ok((cols, _rows)) = size() {
        if let Ok(y) = u16::try_from(x) {
            let status: &str = "[ ok ]";
            if let Ok(len) = u16::try_from(status.len()) {
                let status_position: u16 = cols.saturating_sub(len);
                assert!(
                    execute!(
                        stdout(),
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
                    stdout(),
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
pub fn ko(description: &str, x: usize) {
    if let Ok((cols, _row)) = size() {
        if let Ok(y) = u16::try_from(x) {
            let status: &str = "[ !! ]";
            if let Ok(len) = u16::try_from(status.len()) {
                let status_position: u16 = cols.saturating_sub(len);
                assert!(
                    execute!(
                        stdout(),
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
            execute!(stdout(), Print(description)).is_ok(),
            "Failed to print error message"
        );
    }
}
///
///
/// # Exec
///
/// Execute the command writted in toml
///
/// # Panics
///
/// On crossterm failure
///
/// # Errors
///
/// On check failure
///
pub fn waiting(
    data: (
        String, // title 0
        String, // success 1
        String, // failure 2
    ),
    cmd: &mut Command,
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
                    stdout(),
                    MoveTo(0, y),
                    SetForegroundColor(Color::Green),
                    Print("*"),
                    MoveTo(2, y),
                    SetForegroundColor(Color::White),
                    Print(data.0.to_string()),
                    MoveTo(status_position, y),
                    SetForegroundColor(Color::White),
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
                                    Print(data.0.to_string()),
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
                                sleep(Duration::from_millis(500));
                            }
                        }
                    }
                });

                let command_output = cmd.spawn()?.wait()?.success();

                spinner_done.store(true, Ordering::SeqCst);
                spinner_thread.join().unwrap();
                assert!(
                    crossterm::execute!(stdout(), MoveTo(0, y), Clear(ClearType::CurrentLine))
                        .is_ok()
                );

                return if command_output {
                    ok(data.1.as_str(), x);
                    Ok(())
                } else {
                    ko(data.2.as_str(), x);
                    Err(Error::new(std::io::ErrorKind::Other, "Command failed"))
                };
            }
        }
    }
    Err(Error::new(std::io::ErrorKind::Other, "Error encountered"))
}
