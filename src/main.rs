mod file_parsin;
mod opener;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if is_created() {
        let _ = opener::file_reader::opener();
    }

    Ok(())
}

fn is_created() -> bool {
    return true;
}
