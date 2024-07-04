pub trait Task {
    fn execute(self);
    fn dependencies(self) -> Vec<Box<dyn Task>>;
}
