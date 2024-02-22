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

// The following crates are required to use println
extern crate geranium_rt;
use geranium_rt::{print, println};

// Anything formatable by classic println! also works
// (ie Display traits)
#[derive(Debug)]
struct TupleStruct(char, u32);


#[no_mangle]
fn init() {
}


#[no_mangle]
fn main() {
    let i = -232;
    let j = 666_u32;
    let k = 42_u32;

    let my_led = ('D', 12); // Built-in green led
    let my_ledst = TupleStruct('A', 12_u32);

    // Our print macros cannot capture surrounding variable
    // println!("println! - my_led tuple {my_led:?}"); // this wouldn't work
    println!("println! - my_led tuple {:?}", my_led); // this does work
    println!("println! - my_ledst {:?}", my_ledst);


    print!("print! - print! followed by println!()");
    println!();
    println!("println! - only string litteral");
    println!("println! - string literral with format, i: {}, j: {}, k: {}", i, j, k);
    println!("println! - now testing print! macro");
    print!("print! - only string litteral ");
    print!("print! - string literral with format, i: {}, j: {}, k: {} ", i, j, k);

    loop {
    }
}
