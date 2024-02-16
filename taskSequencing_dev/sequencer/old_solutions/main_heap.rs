
extern crate alloc;
use alloc::boxed::Box;
use embedded_alloc::Heap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

//mod tasks;

pub trait Task {
    fn execute(&mut self) -> (); 
    fn init(&mut self) -> () {}
}

pub struct OrdoTask {
    pub name: *const str,
    pub task: Box<dyn Task>
}

pub struct Job{
    pub task_index: usize,
    pub start: i32,
    pub duration: i32
}

pub type Tasks = Box<[OrdoTask]>;
pub type Jobs = Box<[Job]>;

pub struct Sequencer {
    pub tasks: Tasks,
    pub jobs: Jobs
}

pub struct Task1 {
    pub count: i32
}

impl Task for Task1 {
    fn execute(&mut self) -> () {
        //println!("I am Task 1. Count : {}", self.count);
    }
}

pub struct Task2 {}

impl Task for Task2 {
    fn execute(&mut self) -> () {
        //println!("I am Task 2");
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

//TODO : renommer en construct_tasks
pub fn init_tasks(seq: &mut Sequencer) -> () {
    
    seq.tasks = Box::new(    [
        OrdoTask {name: "Tache 1", task: Box::new(Task1 {count: 12})},
        OrdoTask {name: "Tache 2", task: Box::new(Task2 {})}
    ]);


    seq.jobs = Box::new([
        Job{task_index: 0, duration: 10, start: 7}
    ]);
}

//wait until specified time, and then resets the timer
fn await_(time: i32){
    //while (TIMX_CNT < time);
    //TIMX_CNT = 0;

    //solution alternative : appeler await avec un délai avant l'éxécution de la tache qui a ce temps pour s'éxécuter
    //while ()
    //TIMX_CCR1 = time
    
}

fn main() {
    //println!("Hello, world!");

    let mut seq: Sequencer = Sequencer { tasks: Default::default(), jobs: Default::default() };

    init_tasks(&mut seq);

    //let sequencer: Sequencer = init_tasks();
    for task in seq.tasks.iter_mut() {
        //println!("{}", task.name);
        task.task.init();
    }

    let mut time: i32 = 0;
    for job in seq.jobs.iter() {
        await_(job.start - time);
        let task: &mut OrdoTask = &mut seq.tasks[job.task_index];
        task.task.execute();
        time = job.start;
    }

    //println!("{}", task.name);
    //(task.f)();
}