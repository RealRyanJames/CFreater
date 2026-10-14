use std::io::{Read, stdin};

mod file_parsin;
mod opener;
mod paths;
mod projects;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if is_created() {
        let _ = opener::file_reader::opener();
        stdin().read(&mut [0u8]).unwrap();
    }

    Ok(())
}

fn is_created() -> bool {
    return true;
}
