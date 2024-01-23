// 
type TaskFunction = fn() -> ();

pub struct Task {
    pub name: String,
    pub f: TaskFunction,    
}

pub struct Job <'a> {
    pub task: &'a Task,
    pub start: i32,
    pub duration
}

pub struct JobPreemptive <'a> {
    pub task: &'a Task,
    pub activations: [i32],
    pub duration
}