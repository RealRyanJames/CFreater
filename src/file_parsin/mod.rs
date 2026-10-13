#[warn(dead_code)]

pub struct FileCreated<T> {
    pub file: T,
}

pub trait CreatedByArgs {
    fn check(&mut self) -> Self;
}

impl CreatedByArgs for FileCreated<&str> {
    fn check(&mut self) -> Self {
        let f = FileCreated { file: self.file };

        f
    }
}
