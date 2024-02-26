#![no_std]
#![no_main]
/**
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
**/

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::delay::*;


// Declare a LED tuple
const MY_LED: (char, u32) = ('D', 12); // Built-in green led


#[no_mangle]
fn init() {
    // delay functions must be initialized first. They use TIM2.
    delay_init_timers();
    
    // Enable the GPIO that the LED will be on.
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    
    // Initialize the LED
    gpiod_moder_set(MY_LED.1*2, 2, GPIO_MODER_OUT);
    
    // Set it to LOW by default
    digital_write(MY_LED, LOW);
}


#[no_mangle]
fn main() {
    loop {
        digital_write(MY_LED, LOW);
        delay_ms(1000);
        digital_write(MY_LED, HIGH);
        delay_ms(1000);
    }
}