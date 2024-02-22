use core::cell::RefCell;

use crate::sequencer::task::Task;
use geranium_rt::stm32rustlib::seq::*;

pub struct OrdoTask<'a> {
    pub task: &'a mut dyn Task,
    pub duration: u32, // TODO check if wait is inclusive or exclusive
}

pub struct Job<'a> {
    pub ordo_task: &'a RefCell<OrdoTask<'a>>,
    pub start: u32,
}

// If task runs longer than max_time this function panic!()
pub fn run_task(ordo_task: &mut OrdoTask, max_time: u32) {
    seq_timer_arm_ms_interrupt(max_time);
    ordo_task.task.execute();
    seq_disable_arm_interrupt();
    seq_timer_timeout();
}

pub fn run_sequencer(jobs: &[Job], hyperperiod: u32) -> ! {
    if jobs.is_empty() {
        loop {}
    }

    seq_delay_init_timers();
    seq_timer_arm_interrupt_init();

    if jobs.len() == 1 {
        let job = &jobs[0];
        loop {
            seq_delay_ms(job.start);
            let mut ordo_task = job.ordo_task.borrow_mut();
            let duration = ordo_task.duration;

            run_task(&mut ordo_task, duration);
            seq_delay_ms(hyperperiod - (job.start + duration));
        }
    }

    // At least 2 jobs
    loop {
        seq_delay_ms(jobs[0].start);

        let mut i: usize = 0;
        while i < jobs.len() - 1 {
            let job = &jobs[i];
            let next_job = &jobs[i + 1];
            let mut ordo_task = job.ordo_task.borrow_mut();
            let duration = ordo_task.duration;

            run_task(&mut ordo_task, duration);
            seq_delay_ms(next_job.start - (job.start + duration));

            i += 1;
        }
        let job = &jobs[i];
        let mut ordo_task = job.ordo_task.borrow_mut();
        let duration = ordo_task.duration;

        run_task(&mut ordo_task, duration);
        seq_delay_ms(hyperperiod - (job.start + duration));
    }
}

pub fn init_tasks(ordo_tasks: &mut [&RefCell<OrdoTask>]) {
    for ordo_task in ordo_tasks.iter_mut() {
        ordo_task.borrow_mut().task.init();
    }
}
