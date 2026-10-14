use std::io::{Read, stdin};

mod file_parsin;
mod golang;
mod opener;
mod paths;
mod structs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match is_created() == true {
        true => {
            let _ = opener::file_reader::opener();
            println!("{}", "Close App".to_lowercase());
            println!("{}", "Run Cargo run".to_lowercase());
            stdin().read(&mut [0u8]).unwrap();
        }

        false => println!("{}", "failed"),
    }

    Ok(())
}
fn is_created() -> bool {
    return true;
}
