#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::wait::*;

const APB1_CLK: u32 = 42_000_000;

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
    pub start: u32,
    pub duration: u32
}

pub struct Task1 {
    pub count: u32
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

fn runTask(ordo_task: &mut OrdoTask, max_time: u32){
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    timer_timeout();
}

fn run_sequencer(ordo_tasks: &mut [OrdoTask], num_ordo_tasks: usize, jobs: &[Job], num_jobs: usize, hyperperiod: u32) -> !{
    for task in ordo_tasks.iter_mut() {
        //println!("{}", task.name);
        task.task.init();
    }

    loop {
        let mut i: usize = 0;
        while i < jobs.len() - 1 {
            let job = &jobs[i];
            let next_job = &jobs[i + 1];
            let ordo_task: &mut OrdoTask = &mut ordo_tasks[job.task_index];
    
            runTask(ordo_task, next_job.start - job.start);
    
            i += 1;
        }
        let job = &jobs[i];
        let ordo_task: &mut OrdoTask = &mut ordo_tasks[job.task_index];
    
        runTask(ordo_task, hyperperiod - job.start);
    }
}

//generated code section

pub struct Sequencer <'a> {
    pub tasks: [OrdoTask<'a> ; 2],
    pub jobs: [Job ; 3]
}

#[no_mangle]
fn main() {
    //println!("Hello, world!");

    let mut t1: Task1 = Task1::new();
    let mut t2: Task2 = Task2::new();

    let hyperperiod: u32 = 100;

    let mut ordo_tasks = [
        OrdoTask {name: "Tache 1", task: &mut t1},
        OrdoTask {name: "Tache 2", task: &mut t2}
    ];
    let num_ordo_tasks = 2;

    let jobs = [
        Job{task_index: 0, duration: 10, start: 7},
        Job{task_index: 0, duration: 10, start: 50},
        Job{task_index: 1, duration: 20, start: 20}
    ];
    let num_jobs = 3;

    run_sequencer(&mut ordo_tasks, num_ordo_tasks, &jobs, num_jobs, hyperperiod);

    /*
    let mut time: u32 = 0;
    for job in seq.jobs.iter() {
        wait_ms(job.start - time);
        let task: &mut OrdoTask = &mut seq.tasks[job.task_index];
        task.task.execute();
        time = job.start;
    }

    */
    //println!("{}", task.name);
    //(task.f)();
}