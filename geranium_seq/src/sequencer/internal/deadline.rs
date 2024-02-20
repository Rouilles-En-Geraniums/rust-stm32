extern crate geranium_rt;
use core::cell::RefCell;

use crate::sequencer::task::Task;
use geranium_rt::stm32rustlib::delay::*;


pub struct OrdoTask <'a>{
    pub task: &'a mut dyn Task,
    pub duration: u32 // TODO check if wait is inclusive or exclusive
}

pub struct Job<'a>{
    pub ordo_task: &'a RefCell<OrdoTask<'a>>,
    pub start: u32,
}

pub fn run_task(ordo_task: &mut OrdoTask, max_time: u32){
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    // un-arm interupt
}

pub fn run_sequencer(jobs: &[Job], hyperperiod: u32) -> !{
    if jobs.is_empty() { loop {} }

    // TODO needed ? delay_init_timers();
    // TODO init interrupt (may need helper function from delay)

    if jobs.len() == 1 {
        let job = &jobs[0];
        loop {
            delay_ms(job.start);
            run_task(&mut job.ordo_task.borrow_mut(), hyperperiod - job.start);
        }
    }

    // At least 2 jobs
    loop {
        delay_ms(jobs[0].start);

        let mut i: usize = 0;
        while i < jobs.len() - 1 {
            let job = &jobs[i];
            let next_job = &jobs[i + 1];
            run_task(&mut job.ordo_task.borrow_mut(), next_job.start - job.start);

            i += 1;
        }
        let job = &jobs[i];
        run_task(&mut job.ordo_task.borrow_mut(), hyperperiod - job.start);

    }
}

pub fn init_tasks(ordo_tasks: &mut [& RefCell<OrdoTask>]) {
    for ordo_task in ordo_tasks.iter_mut() {
        ordo_task.borrow_mut().task.init();
    }
}


