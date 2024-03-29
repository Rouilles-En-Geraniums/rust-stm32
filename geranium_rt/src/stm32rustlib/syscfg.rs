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

const SYSCFG_ADR: u32 = 0x40013800;
        
const SYSCFG_MEMRMP_OFFSET: u32 = 0x00;
const SYSCFG_PMC_OFFSET: u32 = 0x04;
const SYSCFG_EXTICR1_OFFSET: u32 = 0x08;
const SYSCFG_EXTICR2_OFFSET: u32 = 0x0C;
const SYSCFG_EXTICR3_OFFSET: u32 = 0x10;
const SYSCFG_EXTICR4_OFFSET: u32 = 0x14;
const SYSCFG_CMPCR_OFFSET: u32 = 0x20;
    
pub const SYSCFG_MEMRMP_MEM_MODE: u32 = 1 << 0;
pub const SYSCFG_PMC_MII_RMII_SEL: u32 = 1 << 23;
pub const SYSCFG_EXTICR1_EXTI0: u32 = 1 << 0;
pub const SYSCFG_EXTICR1_EXTI1: u32 = 1 << 4;
pub const SYSCFG_EXTICR1_EXTI2: u32 = 1 << 8;
pub const SYSCFG_EXTICR1_EXTI3: u32 = 1 << 12;
pub const SYSCFG_EXTICR2_EXTI4: u32 = 1 << 0;
pub const SYSCFG_EXTICR2_EXTI5: u32 = 1 << 4;
pub const SYSCFG_EXTICR2_EXTI6: u32 = 1 << 8;
pub const SYSCFG_EXTICR2_EXTI7: u32 = 1 << 12;
pub const SYSCFG_EXTICR3_EXTI8: u32 = 1 << 0;
pub const SYSCFG_EXTICR3_EXTI9: u32 = 1 << 4;
pub const SYSCFG_EXTICR3_EXTI10: u32 = 1 << 8;
pub const SYSCFG_EXTICR3_EXTI11: u32 = 1 << 12;
pub const SYSCFG_EXTICR4_EXTI12: u32 = 1 << 0;
pub const SYSCFG_EXTICR4_EXTI13: u32 = 1 << 4;
pub const SYSCFG_EXTICR4_EXTI14: u32 = 1 << 8;
pub const SYSCFG_EXTICR4_EXTI15: u32 = 1 << 12;
pub const SYSCFG_CMPCR_CMP_PD: u32 = 1 << 0;
pub const SYSCFG_CMPCR_READY: u32 = 1 << 8;
        
#[inline(always)]
pub fn syscfg_memrmp_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_pmc_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_PMC_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_exticr1_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_EXTICR1_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_exticr2_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_EXTICR2_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_exticr3_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_EXTICR3_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_exticr4_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_EXTICR4_OFFSET) as *mut u32, value) };
}
#[inline(always)]
pub fn syscfg_cmpcr_write(value: u32) {
    unsafe { write_volatile((SYSCFG_ADR + SYSCFG_CMPCR_OFFSET) as *mut u32, value) };
}
    
        
#[inline(always)]
pub fn syscfg_memrmp_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_MEMRMP_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_pmc_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_PMC_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_exticr1_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_EXTICR1_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_exticr2_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_EXTICR2_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_exticr3_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_EXTICR3_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_exticr4_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_EXTICR4_OFFSET) as *mut u32) }
}
#[inline(always)]
pub fn syscfg_cmpcr_read() -> u32 {
    unsafe { read_volatile((SYSCFG_ADR + SYSCFG_CMPCR_OFFSET) as *mut u32) }
}
    
        
#[inline(always)]
pub fn syscfg_memrmp_set(position: u32, size: u32, value: u32) {
    syscfg_memrmp_write(rep_bits(syscfg_memrmp_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_pmc_set(position: u32, size: u32, value: u32) {
    syscfg_pmc_write(rep_bits(syscfg_pmc_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_exticr1_set(position: u32, size: u32, value: u32) {
    syscfg_exticr1_write(rep_bits(syscfg_exticr1_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_exticr2_set(position: u32, size: u32, value: u32) {
    syscfg_exticr2_write(rep_bits(syscfg_exticr2_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_exticr3_set(position: u32, size: u32, value: u32) {
    syscfg_exticr3_write(rep_bits(syscfg_exticr3_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_exticr4_set(position: u32, size: u32, value: u32) {
    syscfg_exticr4_write(rep_bits(syscfg_exticr4_read(), position, size, value));
}
#[inline(always)]
pub fn syscfg_cmpcr_set(position: u32, size: u32, value: u32) {
    syscfg_cmpcr_write(rep_bits(syscfg_cmpcr_read(), position, size, value));
}
    
        
#[inline(always)]
pub fn syscfg_memrmp_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_memrmp_write(syscfg_memrmp_read() | value),
        31 => syscfg_memrmp_write(syscfg_memrmp_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_pmc_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_pmc_write(syscfg_pmc_read() | value),
        31 => syscfg_pmc_write(syscfg_pmc_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_exticr1_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_exticr1_write(syscfg_exticr1_read() | value),
        31 => syscfg_exticr1_write(syscfg_exticr1_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_exticr2_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_exticr2_write(syscfg_exticr2_read() | value),
        31 => syscfg_exticr2_write(syscfg_exticr2_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_exticr3_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_exticr3_write(syscfg_exticr3_read() | value),
        31 => syscfg_exticr3_write(syscfg_exticr3_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_exticr4_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_exticr4_write(syscfg_exticr4_read() | value),
        31 => syscfg_exticr4_write(syscfg_exticr4_read() & value),
        _ => (),
    }
}
#[inline(always)]
pub fn syscfg_cmpcr_seti(value: u32) {
    match value.count_ones() {
        1 => syscfg_cmpcr_write(syscfg_cmpcr_read() | value),
        31 => syscfg_cmpcr_write(syscfg_cmpcr_read() & value),
        _ => (),
    }
}
    