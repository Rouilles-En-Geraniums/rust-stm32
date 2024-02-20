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

use geranium_seq::sequencer::task::*;
use geranium_rt::stm32rustlib::gpio::digital_write;
use geranium_rt::stm32rustlib::various::HIGH;
use geranium_rt::stm32rustlib::various::LOW;

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