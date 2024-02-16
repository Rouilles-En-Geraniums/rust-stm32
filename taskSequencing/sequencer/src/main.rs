#![no_std]
#![no_main]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::delay::*;

//User exposed library

pub trait Task {
    fn execute(&mut self) -> (); 
    fn init(&mut self) -> () {}
    fn new() -> Self where Self: Sized;
}

//Internal library

pub struct OrdoTask <'a>{
    pub name: *const str,
    pub task: &'a mut dyn Task
}

pub struct Job{
    pub task_index: usize,
    pub start: u32,
    pub duration: u32
}

fn run_task(ordo_task: &mut OrdoTask, max_time: u32){
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    timer_timeout();
}

fn run_sequencer(ordo_tasks: &mut [OrdoTask], jobs: &[Job], hyperperiod: u32) -> !{
    for task in ordo_tasks.iter_mut() {
        //println!("{}", task.name);
        task.task.init();
    }

    loop {
        if jobs.len() < 1 {
            continue;
        }

        delay_ms(jobs[1].start);

        let mut i: usize = 0;
        while i < jobs.len() - 1 {
            let job = &jobs[i];
            let next_job = &jobs[i + 1];
            let ordo_task: &mut OrdoTask = &mut ordo_tasks[job.task_index];
    
            run_task(ordo_task, next_job.start - job.start);
    
            i += 1;
        }
        let job = &jobs[i];
        let ordo_task: &mut OrdoTask = &mut ordo_tasks[job.task_index];
    
        run_task(ordo_task, hyperperiod - job.start);
    }
}

//User section

pub fn user_init(){
    delay_init_timers();
    
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);

    gpiod_moder_set(MY_LED.1*2, 2, GPIO_MODER_OUT);
}

const MY_LED: (char, u32) = ('D', 12); // Built-in green led

pub struct LedOn {
}

impl Task for LedOn {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, HIGH);
    }

    fn new() -> LedOn {
        LedOn { }
    }
}

pub struct LedOff {}

impl Task for LedOff {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, LOW);
    }

    fn new() -> LedOff {
        LedOff { }
    }
}

//réfléchir à la possibiltié de laisser l'utilisateur écrire ordo_tab.rs lui-même, avec des helpers (add_task -> OrdoTask, add_job)

//generated code section

#[no_mangle]
fn main() {

    let mut t1: LedOn = LedOn::new();
    let mut t2: LedOff = LedOff::new();

    let hyperperiod: u32 = 2000;

    let mut ordo_tasks = [
        OrdoTask {name: "Led On", task: &mut t1},
        OrdoTask {name: "Led Off", task: &mut t2}
    ];

    let jobs = [
        Job{task_index: 0, duration: 10, start: 0},
        Job{task_index: 1, duration: 10, start: 1000},
    ];

    user_init();

    run_sequencer(&mut ordo_tasks, &jobs, hyperperiod);

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