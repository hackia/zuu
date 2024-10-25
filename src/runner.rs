use std::{fs::create_dir_all, io::Error};

///
/// # Create
///
/// Create directories storage to store tux output file
///
/// # Errors
///
/// On no write rights
///
pub fn create_zuu() -> Result<(), Error> {
    if create_dir_all("zuu").is_ok() {
        Ok(())
    } else {
        Err(Error::other("Failed to create zuu structure"))
    }
}
