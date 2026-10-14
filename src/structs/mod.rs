struct FinalProjectBase<ProjectName> {
    name: ProjectName,
}

trait ProjectCache {
    fn get_catche_project(&mut self) -> Self;
    fn base(&mut self) -> Self;
}
