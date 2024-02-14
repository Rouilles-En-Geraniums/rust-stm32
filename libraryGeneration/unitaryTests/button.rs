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



const PRESSED: u8 = 1;
const RELEASED: u8 = 0;


#[no_mangle]
fn main() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    
    let my_led = ('D', 12); // Built-in green led
    let my_but = ('A', 0); // Built-in blue button

    gpiod_moder_set(my_led.1*2, GPIO_MODER_OUT);

    gpioa_moder_set(my_but.1*2, GPIO_MODER_IN);
    gpioa_pupdr_set(my_but.1*2, GPIO_PUPDR_PD);

    let mut bstate = RELEASED;
    digital_write(my_led, LOW);
    loop {
        if bstate == RELEASED {
            if digital_read(my_but) == HIGH {
                bstate = PRESSED;
                digital_write(my_led, HIGH);
            }
        } else {
            if digital_read(my_but) == LOW {
                bstate = RELEASED;
                digital_write(my_led, LOW);
            }
        }
    }
}
