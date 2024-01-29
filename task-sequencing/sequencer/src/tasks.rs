#[path = "task.rs"] mod task;

// 
type TaskFunction = fn() -> ();

pub struct OrdoTask {
    pub name: String,
    pub task: Box<dyn task::Task>
}

pub struct Job <'a> {
    pub task: &'a OrdoTask,
    pub start: i32,
    pub duration: i32
}

/*
pub struct JobPreemptive <'a> {
    pub task: &'a OrdoTask,
    pub activations: [i32],
    pub duration: i32
}
*/