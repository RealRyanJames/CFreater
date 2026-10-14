pub struct FinalProjectBase<ProjectName> {
    pub name: ProjectName,
}

pub trait ProjectCache {
    fn get_catche_project(&mut self) -> Self;
}

impl ProjectCache for FinalProjectBase<&str> {
    fn get_catche_project(&mut self) -> Self {
        let b = FinalProjectBase {
            name: "ToolingCLIWorker",
        };

        b
    }
}
