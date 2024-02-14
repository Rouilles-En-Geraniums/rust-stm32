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

const DAC_ADR : u32 = 0x40007400;
        
const DAC_CR_OFFSET : u32 = 0x00;
const DAC_SWTRIGR_OFFSET : u32 = 0x04;
const DAC_DHR12R1_OFFSET : u32 = 0x08;
const DAC_DHR12L1_OFFSET : u32 = 0x0C;
const DAC_DHR8R1_OFFSET : u32 = 0x10;
const DAC_DHR12R2_OFFSET : u32 = 0x14;
const DAC_DHR12L2_OFFSET : u32 = 0x18;
const DAC_DHR8R2_OFFSET : u32 = 0x1C;
const DAC_DHR12RD_OFFSET : u32 = 0x20;
const DAC_DHR12LD_OFFSET : u32 = 0x24;
const DAC_DHR8RD_OFFSET : u32 = 0x28;
const DAC_DOR1_OFFSET : u32 = 0x2C;
const DAC_DOR2_OFFSET : u32 = 0x30;
const DAC_SR_OFFSET : u32 = 0x34;
    
pub const DAC_CR_EN : u32 = 1 << 0;
pub const DAC_CR_BOFF1 : u32 = 1 << 1;
pub const DAC_CR_TEN1 : u32 = 1 << 2;
pub const DAC_CR_TSEL1 : u32 = 1 << 3;
pub const DAC_CR_WAVE1 : u32 = 1 << 6;
pub const DAC_CR_MAMP1 : u32 = 1 << 8;
pub const DAC_CR_DMAEN1 : u32 = 1 << 12;
pub const DAC_CR_DMAUDRIE1 : u32 = 1 << 13;
pub const DAC_CR_EN2 : u32 = 1 << 16;
pub const DAC_CR_BOFF2 : u32 = 1 << 17;
pub const DAC_CR_TEN2 : u32 = 1 << 18;
pub const DAC_CR_TSEL2 : u32 = 1 << 19;
pub const DAC_CR_WAVE2 : u32 = 1 << 22;
pub const DAC_CR_MAMP2 : u32 = 1 << 24;
pub const DAC_CR_DMAEN2 : u32 = 1 << 28;
pub const DAC_CR_DMAUDRIE2 : u32 = 1 << 29;
pub const DAC_SWTRIGR_SWTRIG1 : u32 = 1 << 0;
pub const DAC_SWTRIGR_SWTRIG2 : u32 = 1 << 1;
pub const DAC_DHR12R1_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR12L1_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR8R1_DACC2DHR : u32 = 1 << 0;
pub const DAC_DHR12R2_DACC2DHR : u32 = 1 << 4;
pub const DAC_DHR12RD_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR12RD_DACC2DHR : u32 = 1 << 16;
pub const DAC_DHR12LD_DACC1DHR : u32 = 1 << 4;
pub const DAC_DHR12LD_DACC2DHR : u32 = 1 << 16;
pub const DAC_DHR8RD_DACC1DHR : u32 = 1 << 0;
pub const DAC_DHR8RD_DACC2DHR : u32 = 1 << 8;
pub const DAC_DOR1_DACC1DOR : u32 = 1 << 0;
pub const DAC_DOR2_DACC2DOR : u32 = 1 << 0;
pub const DAC_SR_DMAUDR1 : u32 = 1 << 13;
pub const DAC_SR_DMAUDR2 : u32 = 1 << 29;
        
pub fn dac_cr_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_CR_OFFSET) as *mut u32, value)
    };
}
pub fn dac_swtrigr_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_SWTRIGR_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12r1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12R1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12l1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12L1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8r1_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8R1_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12r2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12R2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12l2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12L2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8r2_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8R2_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12rd_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12RD_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr12ld_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR12LD_OFFSET) as *mut u32, value)
    };
}
pub fn dac_dhr8rd_write(value: u32) {
    unsafe {
        write_volatile( (DAC_ADR + DAC_DHR8RD_OFFSET) as *mut u32, value)
    };
}



    
        
pub fn dac_cr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_CR_OFFSET) as *mut u32)
    }
}
pub fn dac_swtrigr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_SWTRIGR_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12r1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12R1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12l1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12L1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8r1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8R1_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12r2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12R2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12l2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12L2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8r2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8R2_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12rd_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12RD_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr12ld_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR12LD_OFFSET) as *mut u32)
    }
}
pub fn dac_dhr8rd_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DHR8RD_OFFSET) as *mut u32)
    }
}
pub fn dac_dor1_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DOR1_OFFSET) as *mut u32)
    }
}
pub fn dac_dor2_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_DOR2_OFFSET) as *mut u32)
    }
}
pub fn dac_sr_read() -> u32 {
    unsafe {
        read_volatile( (DAC_ADR + DAC_SR_OFFSET) as *mut u32)
    }
}
    
        
pub fn dac_cr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_cr_write(rep_bits(dac_cr_read(), position, size, value));
}
pub fn dac_swtrigr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_swtrigr_write(rep_bits(dac_swtrigr_read(), position, size, value));
}
pub fn dac_dhr12r1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12r1_write(rep_bits(dac_dhr12r1_read(), position, size, value));
}
pub fn dac_dhr12l1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12l1_write(rep_bits(dac_dhr12l1_read(), position, size, value));
}
pub fn dac_dhr8r1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr8r1_write(rep_bits(dac_dhr8r1_read(), position, size, value));
}
pub fn dac_dhr12r2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12r2_write(rep_bits(dac_dhr12r2_read(), position, size, value));
}
pub fn dac_dhr12l2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12l2_write(rep_bits(dac_dhr12l2_read(), position, size, value));
}
pub fn dac_dhr8r2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr8r2_write(rep_bits(dac_dhr8r2_read(), position, size, value));
}
pub fn dac_dhr12rd_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12rd_write(rep_bits(dac_dhr12rd_read(), position, size, value));
}
pub fn dac_dhr12ld_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr12ld_write(rep_bits(dac_dhr12ld_read(), position, size, value));
}
pub fn dac_dhr8rd_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    dac_dhr8rd_write(rep_bits(dac_dhr8rd_read(), position, size, value));
}



    
        
pub fn dac_cr_seti(value: u32) {
    match value.count_ones() {
        1 => dac_cr_write(dac_cr_read() | value),
        31 => dac_cr_write(dac_cr_read() & value),
        _ => (),
    }

    
}
pub fn dac_swtrigr_seti(value: u32) {
    match value.count_ones() {
        1 => dac_swtrigr_write(dac_swtrigr_read() | value),
        31 => dac_swtrigr_write(dac_swtrigr_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12r1_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12r1_write(dac_dhr12r1_read() | value),
        31 => dac_dhr12r1_write(dac_dhr12r1_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12l1_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12l1_write(dac_dhr12l1_read() | value),
        31 => dac_dhr12l1_write(dac_dhr12l1_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr8r1_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr8r1_write(dac_dhr8r1_read() | value),
        31 => dac_dhr8r1_write(dac_dhr8r1_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12r2_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12r2_write(dac_dhr12r2_read() | value),
        31 => dac_dhr12r2_write(dac_dhr12r2_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12l2_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12l2_write(dac_dhr12l2_read() | value),
        31 => dac_dhr12l2_write(dac_dhr12l2_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr8r2_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr8r2_write(dac_dhr8r2_read() | value),
        31 => dac_dhr8r2_write(dac_dhr8r2_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12rd_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12rd_write(dac_dhr12rd_read() | value),
        31 => dac_dhr12rd_write(dac_dhr12rd_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr12ld_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr12ld_write(dac_dhr12ld_read() | value),
        31 => dac_dhr12ld_write(dac_dhr12ld_read() & value),
        _ => (),
    }

    
}
pub fn dac_dhr8rd_seti(value: u32) {
    match value.count_ones() {
        1 => dac_dhr8rd_write(dac_dhr8rd_read() | value),
        31 => dac_dhr8rd_write(dac_dhr8rd_read() & value),
        _ => (),
    }

    
}



    