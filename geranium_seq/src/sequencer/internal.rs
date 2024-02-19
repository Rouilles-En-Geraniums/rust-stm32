extern crate geranium_rt;

use crate::sequencer::task::Task;
use geranium_rt::stm32rustlib::delay::timer_arm_ms;
use geranium_rt::stm32rustlib::delay::timer_timeout;
use geranium_rt::stm32rustlib::delay::delay_ms;


pub struct OrdoTask <'a>{
    pub name: *const str,
    pub task: &'a mut dyn Task
}

pub struct Job{
    pub task_index: usize,
    pub start: u32,
    pub duration: u32
}

pub fn run_task(ordo_task: &mut OrdoTask, max_time: u32){
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    timer_timeout();
}

pub fn run_sequencer(ordo_tasks: &mut [OrdoTask], jobs: &[Job], hyperperiod: u32) -> !{
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
