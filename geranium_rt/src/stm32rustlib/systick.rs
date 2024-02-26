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
use crate::stm32rustlib::various::*;

const SYSTICK_ADR: u32 = 0xE000E010;
        
const SYSTICK_CSR_OFFSET: u32 = 0x00;
const SYSTICK_RVR_OFFSET: u32 = 0x04;
const SYSTICK_CVR_OFFSET: u32 = 0x08;
const SYSTICK_CALIB_OFFSET: u32 = 0x0C;
    
pub const SYSTICK_CSR_COUNTFLAG: u32 = 1 << 16;
pub const SYSTICK_CSR_CLKSOURCE: u32 = 1 << 2;
pub const SYSTICK_CSR_TICKINT: u32 = 1 << 1;
pub const SYSTICK_CSR_ENABLE: u32 = 1 << 0;
        
#[inline(always)]
pub fn systick_csr_write(value: u32) {
    unsafe { write_volatile((SYSTICK_ADR + SYSTICK_CSR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn systick_rvr_write(value: u32) {
    unsafe { write_volatile((SYSTICK_ADR + SYSTICK_RVR_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn systick_cvr_write(value: u32) {
    unsafe { write_volatile((SYSTICK_ADR + SYSTICK_CVR_OFFSET) as *mut u32, value) };
}

    
        
#[inline(always)]
pub fn systick_csr_read() -> u32 {
    unsafe { read_volatile((SYSTICK_ADR + SYSTICK_CSR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn systick_rvr_read() -> u32 {
    unsafe { read_volatile((SYSTICK_ADR + SYSTICK_RVR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn systick_cvr_read() -> u32 {
    unsafe { read_volatile((SYSTICK_ADR + SYSTICK_CVR_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn systick_calib_read() -> u32 {
    unsafe { read_volatile((SYSTICK_ADR + SYSTICK_CALIB_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn systick_csr_set(position: u32, size: u32, value: u32) {
    systick_csr_write(rep_bits(systick_csr_read(), position, size, value));
}
#[inline(always)]
pub fn systick_rvr_set(position: u32, size: u32, value: u32) {
    systick_rvr_write(rep_bits(systick_rvr_read(), position, size, value));
}
#[inline(always)]
pub fn systick_cvr_set(position: u32, size: u32, value: u32) {
    systick_cvr_write(rep_bits(systick_cvr_read(), position, size, value));
}

    
        
#[inline(always)]
pub fn systick_csr_seti(value: u32) {
    match value.count_ones() {
        1 => systick_csr_write(systick_csr_read() | value),
        31 => systick_csr_write(systick_csr_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn systick_rvr_seti(value: u32) {
    match value.count_ones() {
        1 => systick_rvr_write(systick_rvr_read() | value),
        31 => systick_rvr_write(systick_rvr_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn systick_cvr_seti(value: u32) {
    match value.count_ones() {
        1 => systick_cvr_write(systick_cvr_read() | value),
        31 => systick_cvr_write(systick_cvr_read() & value),
        _ => (),
    }
}

    