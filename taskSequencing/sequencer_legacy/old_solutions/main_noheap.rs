#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::tim;

pub mod stm32rustlib;

use embedded_alloc::Heap as Heap;

//#[global_allocator]
//static HEAP: Heap = Heap::empty();

//mod tasks;

pub trait Task {
    fn execute(&mut self) -> (); 
    fn init(&mut self) -> () {}
    fn new() -> Self where Self: Sized;
}

pub struct OrdoTask <'a>{
    pub name: *const str,
    pub task: &'a mut dyn Task
}

pub struct Job{
    pub task_index: usize,
    pub start: i32,
    pub duration: i32
}

pub struct Task1 {
    pub count: i32
}

impl Task for Task1 {
    fn execute(&mut self) -> () {
        //println!("I am Task 1. Count : {}", self.count);
    }

    fn new() -> Task1 {
        return Task1 { count: 32 }
    }
}

pub struct Task2 {}

impl Task for Task2 {
    fn execute(&mut self) -> () {
        //println!("I am Task 2");
    }

    fn new() -> Task2 {
        return Task2 { }
    }
}

//réfléchir à la possibiltié delaisser l'utilisateur écrire ordo_tab.rs lui-même, avec des helpers (add_task -> OrdoTask, add_job)



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

/*
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

*/

//wait until specified time, and then resets the timer
fn await_(time: i32){
    //while (TIMX_CNT < time);
    //TIMX_CNT = 0;

    //solution alternative : appeler await avec un délai avant l'éxécution de la tache qui a ce temps pour s'éxécuter
    //while ()
    //TIMX_CCR1 = time
    
}

//generated code section

pub struct Sequencer <'a> {
    pub tasks: [OrdoTask<'a> ; 2],
    pub jobs: [Job ; 1]
}

fn main() {
    //println!("Hello, world!");

    let mut t1: Task1 = Task1::new();
    let mut t2: Task2 = Task2::new();

    let mut seq: Sequencer = Sequencer { tasks: [
        OrdoTask {name: "Tache 1", task: &mut t1},
        OrdoTask {name: "Tache 2", task: &mut t2}
    ], jobs: [
        Job{task_index: 0, duration: 10, start: 7}
    ] };

    //init_tasks(&mut seq);

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