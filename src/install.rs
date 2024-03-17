use std::process::{exit, Command, ExitCode};

fn main() -> ExitCode {
    print!("{}", ansi_escapes::ClearScreen);
    if Command::new("cargo")
        .arg("install")
        .arg("zuu")
        .spawn()
        .unwrap()
        .wait()
        .is_ok()
    {
        println!("\x1b[1;32m    Finished\x1b[37m zuu installed successfully\x1b[30m");
        exit(0);
    }
    println!("\x1b[1;32m    Finished\x1b[37m zuu install fail\x1b[30m");
    exit(1);
}
