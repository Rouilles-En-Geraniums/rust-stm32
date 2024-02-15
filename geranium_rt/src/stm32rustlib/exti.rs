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
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various::*;

const EXTI_ADR : u32 = 0x40013C00;
        
const EXTI_IMR_OFFSET : u32 = 0x00;
const EXTI_EMR_OFFSET : u32 = 0x04;
const EXTI_RTSR_OFFSET : u32 = 0x08;
const EXTI_FTSR_OFFSET : u32 = 0x0C;
const EXTI_SWIER_OFFSET : u32 = 0x10;
const EXTI_PR_OFFSET : u32 = 0x14;
    
pub const EXTI_IMR_MR : u32 = 1 << 0;
pub const EXTI_EMR_MR : u32 = 1 << 0;
pub const EXTI_RTSR_TR : u32 = 1 << 0;
pub const EXTI_FTSR_TR : u32 = 1 << 0;
pub const EXTI_SWIER_SWIER : u32 = 1 << 0;
pub const EXTI_PR_PR : u32 = 1 << 0;
        
pub fn exti_imr_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_IMR_OFFSET) as *mut u32, value)
    };
}
pub fn exti_emr_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_EMR_OFFSET) as *mut u32, value)
    };
}
pub fn exti_rtsr_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_RTSR_OFFSET) as *mut u32, value)
    };
}
pub fn exti_ftsr_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_FTSR_OFFSET) as *mut u32, value)
    };
}
pub fn exti_swier_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_SWIER_OFFSET) as *mut u32, value)
    };
}
pub fn exti_pr_write(value: u32) {
    unsafe {
        write_volatile( (EXTI_ADR + EXTI_PR_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn exti_imr_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_IMR_OFFSET) as *mut u32)
    }
}
pub fn exti_emr_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_EMR_OFFSET) as *mut u32)
    }
}
pub fn exti_rtsr_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_RTSR_OFFSET) as *mut u32)
    }
}
pub fn exti_ftsr_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_FTSR_OFFSET) as *mut u32)
    }
}
pub fn exti_swier_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_SWIER_OFFSET) as *mut u32)
    }
}
pub fn exti_pr_read() -> u32 {
    unsafe {
        read_volatile( (EXTI_ADR + EXTI_PR_OFFSET) as *mut u32)
    }
}
    
        
pub fn exti_imr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_imr_write(rep_bits(exti_imr_read(), position, size, value));
}
pub fn exti_emr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_emr_write(rep_bits(exti_emr_read(), position, size, value));
}
pub fn exti_rtsr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_rtsr_write(rep_bits(exti_rtsr_read(), position, size, value));
}
pub fn exti_ftsr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_ftsr_write(rep_bits(exti_ftsr_read(), position, size, value));
}
pub fn exti_swier_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_swier_write(rep_bits(exti_swier_read(), position, size, value));
}
pub fn exti_pr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    exti_pr_write(rep_bits(exti_pr_read(), position, size, value));
}
    
        
pub fn exti_imr_seti(value: u32) {
    match value.count_ones() {
        1 => exti_imr_write(exti_imr_read() | value),
        31 => exti_imr_write(exti_imr_read() & value),
        _ => (),
    }


}
pub fn exti_emr_seti(value: u32) {
    match value.count_ones() {
        1 => exti_emr_write(exti_emr_read() | value),
        31 => exti_emr_write(exti_emr_read() & value),
        _ => (),
    }


}
pub fn exti_rtsr_seti(value: u32) {
    match value.count_ones() {
        1 => exti_rtsr_write(exti_rtsr_read() | value),
        31 => exti_rtsr_write(exti_rtsr_read() & value),
        _ => (),
    }


}
pub fn exti_ftsr_seti(value: u32) {
    match value.count_ones() {
        1 => exti_ftsr_write(exti_ftsr_read() | value),
        31 => exti_ftsr_write(exti_ftsr_read() & value),
        _ => (),
    }


}
pub fn exti_swier_seti(value: u32) {
    match value.count_ones() {
        1 => exti_swier_write(exti_swier_read() | value),
        31 => exti_swier_write(exti_swier_read() & value),
        _ => (),
    }


}
pub fn exti_pr_seti(value: u32) {
    match value.count_ones() {
        1 => exti_pr_write(exti_pr_read() | value),
        31 => exti_pr_write(exti_pr_read() & value),
        _ => (),
    }


}
    