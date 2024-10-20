use std::io::Error;
use zuu::{Language, Zuu};

fn main() -> Result<(), Error> {
    Zuu::new().run(&Language::Vlang)
}
