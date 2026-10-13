pub mod file_reader {
    use std::fs::{self, File};

    use crate::file_parsin::FileCreated;

    pub fn opener() -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::Value;
        use std::path::Path;

        let f = Path::new("Args.json");

        let data_file = fs::read_to_string(f)?;
        let data: Value = serde_json::from_str(&data_file)?;

        let d = data["FileName"].as_str().unwrap();
        println!("File Name: {}", d);

        let file_s = FileCreated { file: d };

        File::create(file_s.file)?;

        Ok(())
    }
}
