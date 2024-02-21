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
use crate::core::ptr::read_volatile;
use crate::core::ptr::write_volatile;

pub const HIGH: u8 = 1;
pub const LOW: u8 = 0;

#[inline(always)]
pub fn mask(l: u32) -> u32 {
    (1 << (l)) - 1
}

#[inline(always)]
pub fn get_bits(x: u32, i: u32, l: u32) -> u32 {
    ((x) >> (i)) & mask(l)
}

#[inline(always)]
pub fn rep_bits(x: u32, i: u32, l: u32, y: u32) -> u32 {
    ((x) & !(mask(l) << i)) | ((y) << (i))
}

// Some FLASH variables
pub const FLASH_ADR: u32 = 0x40023C00;
pub const FLASH_ACR_OFFSET: u32 = 0x00;
pub const FLASH_ACR_LATENCY: u32 = 0; //range = 2 bits
pub const FLASH_ACR_DCEN: u32 = 1 << 10;
pub const FLASH_ACR_ICEN: u32 = 1 << 9;

#[inline(always)]
pub fn flash_acr_write(value: u32) {
    unsafe { write_volatile((FLASH_ADR + FLASH_ACR_OFFSET) as *mut u32, value) };
}

#[inline(always)]
pub fn flash_acr_read() -> u32 {
    unsafe { read_volatile((FLASH_ADR + FLASH_ACR_OFFSET) as *mut u32) }
}
