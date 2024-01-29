//mod tasks;

pub trait Task {
    fn execute(&mut self) -> (); 
    fn init(&mut self) -> () {}
}

pub struct OrdoTask {
    pub name: String,
    pub task: Box<dyn Task>
}

pub struct Job <'a> {
    pub task: &'a OrdoTask,
    pub start: i32,
    pub duration: i32
}

pub struct Sequencer <'a> {
    pub tasks: Vec<OrdoTask>,
    pub jobs: Vec<Job<'a>>
}

pub struct Task1 {
    pub count: i32
}

impl Task for Task1 {
    fn execute(&mut self) -> () {
        println!("I am Task 1. Count : {}", self.count);
    }
}

pub struct Task2 {}

impl Task for Task2 {
    fn execute(&mut self) -> () {
        println!("I am Task 2");
    }
}

//réfléchir à la possibiltié laisser l'utilisateur écrire ordo_tab.rs lui-même, avec des helpers (add_task -> OrdoTask, add_job)



/*
pub fn init_tasks<'a>(tasks: &mut Vec<OrdoTask>, jobs: Vec<Job<'a>>) -> () {
    
    tasks = vec![
        OrdoTask {name: String::from("Tache 1"), task: Box::new(Task1 {count: 12})},
        OrdoTask {name: String::from("Tache 2"), task: Box::new(Task2 {})}
    ]


    jobs = vec![
        Job::<'a>{task: &seq.tasks[1], duration: 10, start: 7}
    ];
}
*/

pub fn init_tasks<'a>(tasks: &'a mut Vec<OrdoTask>, jobs: &'_ mut Vec<Job<'a>>) -> () {
    
    *tasks = vec![
        OrdoTask {name: String::from("Tache 1"), task: Box::new(Task1 {count: 12})},
        OrdoTask {name: String::from("Tache 2"), task: Box::new(Task2 {})}
    ];


    *jobs = vec![
        Job::<'a>{task: &tasks[1], duration: 10, start: 7}
    ];


}

fn main() {
    println!("Hello, world!");

    let mut tasks: Vec<OrdoTask> = vec![];
    let mut jobs: Vec<Job<'_>> = vec![];

    init_tasks(&mut tasks, &mut jobs);

    //let sequencer: Sequencer = init_tasks();
    for task in tasks.iter() {
        println!("{}", task.name);
        task.task.init();
    }

    for job in jobs.iter() {
        job.task.task.execute();
    }

    //println!("{}", task.name);
    //(task.f)();
}