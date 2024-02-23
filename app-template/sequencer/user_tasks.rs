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

use geranium_rt::println;
use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::{HIGH, LOW};
use geranium_seq::sequencer::task::*;

const MY_LED: (char, u32) = ('D', 12); // Built-in green led

// derive Debug (/Display if you have an allocator) to print your struct
#[derive(Debug)]
pub struct LedOn {
    // use variables to share state between calls
    count: u32,
}

impl Task for LedOn {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, HIGH);
        self.count += 1;
        println!("{:?}, count: {}", self, self.count);
    }

    fn new() -> LedOn {
        LedOn { count: 0 }
    }

    fn init(&mut self) {
        println!("init {:?}", self);
        rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
        gpiod_moder_set(MY_LED.1 * 2, 2, GPIO_MODER_OUT);
    }
}

#[derive(Debug)]
pub struct LedOff {
    icount: i32,
}

impl Task for LedOff {
    fn execute(&mut self) -> () {
        digital_write(MY_LED, LOW);
        self.icount += 1;
        println!("{:?}, icount: {}", self, self.icount);
    }

    fn new() -> LedOff {
        LedOff { icount: 0 }
    }

    fn init(&mut self) {
        println!("init {:?}", self);
    }
}
