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

extern crate core;
use core::fmt::Write;
use core::fmt::write;
use core::fmt::Arguments;

use crate::stm32rustlib::gdb::*;

#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::stm32rustlib::geranium_print::print(format_args!($s))
    };
    ($($tt:tt)*) => {
        $crate::stm32rustlib::geranium_print::print(format_args!($($tt)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::stm32rustlib::geranium_print::print(format_args!("\n"))
    };
    ($s:expr) => {
        $crate::stm32rustlib::geranium_print::print(format_args!(concat!($s, "\n")))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::stm32rustlib::geranium_print::print(format_args!(concat!($s, "\n"), $($tt)*))
    };
}

pub fn print(args: Arguments<'_>) {
    let mut p = Printer;
    p.print(args);
}

pub fn println(args: Arguments<'_>) {
    let mut p = Printer;
    p.println(args);
}

struct Printer;

impl Printer {
    fn print(&mut self, args: Arguments<'_>) -> () {
        let _ = write(self, args); //
    }

    fn println(&mut self, args: Arguments<'_>) -> () {
        let _ = write(self, args);
        let _ = self.write_char('\n');
    }
}

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for ch in s.chars() {
            while (itm_stimulus_port0_read() & 1) == 0 {}
            itm_stimulus_port0_write(ch as u32);
        }
        Ok(())
    }
}
