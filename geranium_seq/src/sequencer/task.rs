pub trait Task {
    fn execute(&mut self) -> ();
    fn init(&mut self) -> () {}
    fn new() -> Self
    where
        Self: Sized;
}
