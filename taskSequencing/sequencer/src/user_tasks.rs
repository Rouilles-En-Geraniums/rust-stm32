#[path = "task.rs"] mod task;

pub struct Task1 {
    pub count: i32
}

impl task::Task for Task1 {
    fn execute(&self) -> () {
        println!("I am Task 1. Count : {}", self.count);
    }
}

pub struct Task2 {}

impl task::Task for Task2 {
    fn execute(&self) -> () {
        println!("I am Task 2");
    }
}