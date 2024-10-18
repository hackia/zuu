use crate::utils::{Language, Zuu};
use std::io::Error;

fn main() -> Result<(), Error> {
    Zuu::new().run(&Language::Nodejs)
}
