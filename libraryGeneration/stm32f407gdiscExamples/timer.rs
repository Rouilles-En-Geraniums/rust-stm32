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
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::system::*;

const MY_LED: (char, u32)  = ('D', 12); // Built-in green led

const PSC: u32 = 1000;
const PERIOD: u32 = APB1_CLK / 1000;

pub fn init_timer() {
    tim4_cr1_seti(!TIM_CEN);
    tim4_psc_write(PSC - 1);
    tim4_arr_write(PERIOD);
    tim4_egr_write(TIM_UG);
    tim4_sr_write(0);
    tim4_cr1_seti(TIM_CEN);
}


#[no_mangle]
fn init() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM4EN);
    
    gpiod_moder_set(MY_LED.1*2, 2, GPIO_MODER_OUT);
    
    init_timer();
    
    digital_write(MY_LED, LOW);
}


#[no_mangle]
fn main() {
    loop {
        while (tim4_sr_read() & TIM_UIF) == 0 {};

        if digital_read(MY_LED) == LOW {
			digital_write(MY_LED, HIGH);
		} else {
            digital_write(MY_LED, LOW);
		}
        tim4_sr_write(0);
    }
}