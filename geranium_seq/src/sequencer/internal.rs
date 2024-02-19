extern crate geranium_rt;
use core::cell::RefCell;

use crate::sequencer::task::Task;
use geranium_rt::stm32rustlib::delay::timer_arm_ms;
use geranium_rt::stm32rustlib::delay::timer_timeout;
use geranium_rt::stm32rustlib::delay::delay_ms;


pub struct OrdoTask <'a>{
    pub task: &'a mut dyn Task
}

pub struct Job<'a>{
    pub ordo_task: &'a RefCell<OrdoTask<'a>>,
    pub start: u32,
    pub duration: u32 // TODO check if wait is inclusive or exclusive
}

pub fn run_task(ordo_task: &mut OrdoTask, max_time: u32){
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    timer_timeout();
}

pub fn run_sequencer(jobs: &[Job], hyperperiod: u32) -> !{
    if jobs.is_empty() { loop {} }

    if jobs.len() == 1 {
        let job = &jobs[0];
        loop {
            delay_ms(job.start);
            run_task(&mut job.ordo_task.borrow_mut(), job.duration);
            delay_ms(hyperperiod - (job.start + job.duration ));
        }
    }

    // At least 2 jobs
    loop {
        delay_ms(jobs[0].start);

        let mut i: usize = 0;
        while i < jobs.len() - 1 {
            let job = &jobs[i];
            let next_job = &jobs[i + 1];
            run_task(&mut job.ordo_task.borrow_mut(), job.duration);
            delay_ms(next_job.start - (job.start + job.duration));

            i += 1;
        }
        let job = &jobs[i];
        run_task(&mut job.ordo_task.borrow_mut(), job.duration);
        delay_ms(hyperperiod - (job.start + job.duration));
    }
}

pub fn init_tasks(ordo_tasks: &mut [OrdoTask]) {
    for ordo_task in ordo_tasks.iter_mut() {
        ordo_task.task.init();
    }
}
