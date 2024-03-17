use fs::remove_file;
use std::fs;
use std::process::{exit, ExitCode};
const ZUU: &str = "/usr/bin/zuu";

fn main() -> ExitCode {
    let e = remove_file(ZUU);
    match e {
        Ok(()) => {
            println!("zuu has been removed successfully");
            exit(0);
        }
        Err(x) => {
            println!("{}", x.kind().to_string());
            exit(1);
        }
    }
}
