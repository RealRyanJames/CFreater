use std::path::Path;

use crate::paths::FSync;

#[warn(dead_code)]

pub struct FileCreated<T> {
    pub file: T,
}

pub trait CreatedByArgs {
    fn check(&mut self) -> Self;
}

impl CreatedByArgs for FileCreated<&str> {
    fn check(&mut self) -> Self {
        let path = Path::new(self.file);
        let file_exsits: FSync<&str, bool> = FSync {
            _file_name: self.file,
            file_exists: path.exists(),
        };

        let f = FileCreated { file: self.file };

        if file_exsits.file_exists {
            println!("{}", format!("{}", "Exists Already"))
        }

        f
    }
}
