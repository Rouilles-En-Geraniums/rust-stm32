#![no_std]
#![no_main]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::wait::*;

const APB1_CLK: u32 = 42_000_000;

pub trait Task {
    fn execute(&mut self) -> ();
    fn init(&mut self) -> () {}
    fn new() -> Self
    where
        Self: Sized;
}

pub struct OrdoTask<'a> {
    // pub name: *const str,
    pub task: &'a mut dyn Task,
}

pub struct Job {
    pub task_index: usize,
    pub start: u32,
    pub duration: u32,
}

// User tasks

const MY_LED: (char, u32) = ('D', 12); // Built-in green led

pub struct LedOn {}

impl Task for LedOn {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, HIGH);
    }

    fn new() -> LedOn {
        LedOn {}
    }
}

pub struct LedOff {}

impl Task for LedOff {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, LOW);
    }

    fn new() -> LedOff {
        LedOff {}
    }
}

// Library

fn run_task(ordo_task: &mut OrdoTask, max_time: u32) {
    timer_arm_ms(max_time);
    ordo_task.task.execute();
    timer_timeout();
}

fn run_sequencer(
    ordo_tasks: &mut [OrdoTask],
    num_ordo_tasks: usize,
    jobs: &[Job],
    num_jobs: usize,
    hyperperiod: u32,
) -> ! {
    for task in ordo_tasks.iter_mut() {
        task.task.init();
    }

    loop {
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

#[no_mangle]
fn main() {
    // Start of Generated code
    let hyperperiod: u32 = {{hyperperiod}};

    {%- for task in tasks %}
    let mut t_{{task.id}} = {{task.taskStruct}}::new();
    {%- endfor %}

    {%- for task in tasks %}
    let ot_{{task.id}} = RefCell::new(OrdoTask {
        task: &mut t_{{task.id}},
        duration: {{task.duration}}
    });
    {%- endfor %}


    let jobs = [
    {%- for job in jobs %}
        Job {
            ordo_task: &ot_{{job.taskId}},
            start: {{job.startTime}},
        },
    {%- endfor %}
    ];

    {
        let mut ordo_tasks = [
            {%- for task in tasks %}
            &ot_{{task.id}}
            {%- endfor %}
        ];
        init_tasks(&mut ordo_tasks);
    }

    run_sequencer(
        &jobs,
        hyperperiod
    );
    // End of generated code
}
