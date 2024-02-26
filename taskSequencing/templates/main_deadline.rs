#![no_std]
#![no_main]
/*
 *	Rust on STM32 Project by Rouilles en GeraniumTM
 *	Copyright (C) 2024 Université de Toulouse :
 *   - Oussama Felfel - oussama.felfel@univ-tlse3.fr
 *   - François Foltete - francois.foltete@univ-tlse3.fr
 *   - Elana Courtines - elana.courtines@univ-tlse3.fr
 *   - Teo Tinarrage - teo.tinarrage@univ-tlse3.fr
 *   - Zineb Moubarik - zineb.moubarik@univ-tlse3.fr
 *
 *  This library aims to provide the following :
 *   - a rust library generation tool to safely access memory ;
 *   - a support to flash STM32 boards ;
 *   - a task scheduling tool that generates the associated rust code.
 *
 *  The development of this library has done as a Proof of Concept and
 *  is currently only tested for STM32F407-G DISC1 Boards.
 *
 *  It is our hope that using this library to enable development on
 *  other boards will be facilitated.
 *
 *
 *	This program is free software: you can redistribute it and/or modify
 *	it under the terms of the GNU General Public License as published by
 *	the Free Software Foundation, either version 3 of the License, or
 *	(at your option) any later version.
 *
 *	This program is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *	GNU General Public License for more details.
*/

extern crate geranium_rt;
extern crate geranium_seq;
use core::cell::RefCell;

mod user_tasks;
use user_tasks::*;
use geranium_seq::sequencer::internal::deadline::*;
use geranium_seq::sequencer::task::Task;

#[no_mangle]
fn init(){
    
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
            &ot_{{task.id}},
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
