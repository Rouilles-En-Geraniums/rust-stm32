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

const ADC1_ADR : u32 = 0x40012000;
const ADC2_ADR : u32 = 0x40012100;
const ADC3_ADR : u32 = 0x40012200;
        
const ADC1_SR_OFFSET : u32 = 0x00;
const ADC1_CR1_OFFSET : u32 = 0x04;
const ADC1_CR2_OFFSET : u32 = 0x08;
const ADC1_SMPR1_OFFSET : u32 = 0x0C;
const ADC1_SMPR2_OFFSET : u32 = 0x10;
const ADC1_JOFR1_OFFSET : u32 = 0x14;
const ADC1_HTR_OFFSET : u32 = 0x24;
const ADC1_LTR_OFFSET : u32 = 0x28;
const ADC1_SQR1_OFFSET : u32 = 0x2C;
const ADC1_SQR2_OFFSET : u32 = 0x30;
const ADC1_SQR3_OFFSET : u32 = 0x34;
const ADC1_JSQR_OFFSET : u32 = 0x38;
const ADC1_JDR1_OFFSET : u32 = 0x3C;
const ADC1_DR_OFFSET : u32 = 0x4C;
    
        
const ADC2_SR_OFFSET : u32 = 0x00;
const ADC2_CR1_OFFSET : u32 = 0x04;
const ADC2_CR2_OFFSET : u32 = 0x08;
const ADC2_SMPR1_OFFSET : u32 = 0x0C;
const ADC2_SMPR2_OFFSET : u32 = 0x10;
const ADC2_JOFR1_OFFSET : u32 = 0x14;
const ADC2_HTR_OFFSET : u32 = 0x24;
const ADC2_LTR_OFFSET : u32 = 0x28;
const ADC2_SQR1_OFFSET : u32 = 0x2C;
const ADC2_SQR2_OFFSET : u32 = 0x30;
const ADC2_SQR3_OFFSET : u32 = 0x34;
const ADC2_JSQR_OFFSET : u32 = 0x38;
const ADC2_JDR1_OFFSET : u32 = 0x3C;
const ADC2_DR_OFFSET : u32 = 0x4C;
    
        
const ADC3_SR_OFFSET : u32 = 0x00;
const ADC3_CR1_OFFSET : u32 = 0x04;
const ADC3_CR2_OFFSET : u32 = 0x08;
const ADC3_SMPR1_OFFSET : u32 = 0x0C;
const ADC3_SMPR2_OFFSET : u32 = 0x10;
const ADC3_JOFR1_OFFSET : u32 = 0x14;
const ADC3_HTR_OFFSET : u32 = 0x24;
const ADC3_LTR_OFFSET : u32 = 0x28;
const ADC3_SQR1_OFFSET : u32 = 0x2C;
const ADC3_SQR2_OFFSET : u32 = 0x30;
const ADC3_SQR3_OFFSET : u32 = 0x34;
const ADC3_JSQR_OFFSET : u32 = 0x38;
const ADC3_JDR1_OFFSET : u32 = 0x3C;
const ADC3_DR_OFFSET : u32 = 0x4C;
    
pub const ADC_SR_AWD : u32 = 1 << 16;
pub const ADC_SR_EOC : u32 = 1 << 17;
pub const ADC_SR_JEOC : u32 = 1 << 18;
pub const ADC_SR_JSTRT : u32 = 1 << 19;
pub const ADC_SR_STRT : u32 = 1 << 20;
pub const ADC_SR_OVR : u32 = 1 << 21;
pub const ADC_CR1_AWDCH : u32 = 1 << 0;
pub const ADC_CR1_EOCIE : u32 = 1 << 5;
pub const ADC_CR1_AWDIE : u32 = 1 << 6;
pub const ADC_CR1_JEOCIE : u32 = 1 << 7;
pub const ADC_CR1_SCAN : u32 = 1 << 8;
pub const ADC_CR1_AWDSGL : u32 = 1 << 9;
pub const ADC_CR1_JAUTO : u32 = 1 << 10;
pub const ADC_CR1_DISCEN : u32 = 1 << 11;
pub const ADC_CR1_JDISCEN : u32 = 1 << 12;
pub const ADC_CR1_DISCNUM : u32 = 1 << 13;
pub const ADC_CR1_JAWDEN : u32 = 1 << 22;
pub const ADC_CR1_AWDEN : u32 = 1 << 23;
pub const ADC_CR1_OVRIE : u32 = 1 << 26;
pub const ADC_CR2_ADON : u32 = 1 << 0;
pub const ADC_CR2_CONT : u32 = 1 << 1;
pub const ADC_CR2_DMA : u32 = 1 << 8;
pub const ADC_CR2_DDS : u32 = 1 << 9;
pub const ADC_CR2_EOCS : u32 = 1 << 10;
pub const ADC_CR2_ALIGN : u32 = 1 << 11;
pub const ADC_CR2_JEXTSEL : u32 = 1 << 16;
pub const ADC_CR2_JEXTEN : u32 = 1 << 20;
pub const ADC_CR2_JSWSTART : u32 = 1 << 22;
pub const ADC_CR2_EXTSEL : u32 = 1 << 24;
pub const ADC_CR2_EXTSEN : u32 = 1 << 28;
pub const ADC_CR2_SWSTART : u32 = 1 << 30;
pub const ADC_SMPR1_SMP10 : u32 = 1 << 0;
pub const ADC_SMPR1_SMP11 : u32 = 1 << 3;
pub const ADC_SMPR1_SMP12 : u32 = 1 << 6;
pub const ADC_SMPR1_SMP13 : u32 = 1 << 9;
pub const ADC_SMPR1_SMP14 : u32 = 1 << 12;
pub const ADC_SMPR1_SMP15_0 : u32 = 1 << 15;
pub const ADC_SMPR1_SMP15 : u32 = 1 << 16;
pub const ADC_SMPR1_SMP16 : u32 = 1 << 18;
pub const ADC_SMPR1_SMP17 : u32 = 1 << 21;
pub const ADC_SMPR1_SMP18 : u32 = 1 << 24;
pub const ADC_SMPR2_SMP0 : u32 = 1 << 0;
pub const ADC_SMPR2_SMP1 : u32 = 1 << 3;
pub const ADC_SMPR2_SMP2 : u32 = 1 << 6;
pub const ADC_SMPR2_SMP3 : u32 = 1 << 9;
pub const ADC_SMPR2_SMP4 : u32 = 1 << 12;
pub const ADC_SMPR2_SMP5_0 : u32 = 1 << 15;
pub const ADC_SMPR2_SMP5 : u32 = 1 << 16;
pub const ADC_SMPR2_SMP6 : u32 = 1 << 18;
pub const ADC_SMPR2_SMP7 : u32 = 1 << 21;
pub const ADC_SMPR2_SMP8 : u32 = 1 << 24;
pub const ADC_SMPR2_SMP9 : u32 = 1 << 27;
pub const ADC_JOFR1_JOFFSET1 : u32 = 1 << 0;
pub const ADC_HTR_HT : u32 = 1 << 0;
pub const ADC_LTR_LT : u32 = 1 << 0;
pub const ADC_SQR1_SQ13 : u32 = 1 << 0;
pub const ADC_SQR1_SQ14 : u32 = 1 << 5;
pub const ADC_SQR1_SQ15 : u32 = 1 << 10;
pub const ADC_SQR1_SQ15_0 : u32 = 1 << 15;
pub const ADC_SQR1_SQ16 : u32 = 1 << 16;
pub const ADC_SQR1_L : u32 = 1 << 20;
pub const ADC_SQR2_SQ7 : u32 = 1 << 0;
pub const ADC_SQR2_SQ8 : u32 = 1 << 5;
pub const ADC_SQR2_SQ9 : u32 = 1 << 10;
pub const ADC_SQR2_SQ10_0 : u32 = 1 << 15;
pub const ADC_SQR2_SQ10 : u32 = 1 << 16;
pub const ADC_SQR2_SQ11 : u32 = 1 << 20;
pub const ADC_SQR2_SQ12 : u32 = 1 << 25;
pub const ADC_SQR3_SQ1 : u32 = 1 << 0;
pub const ADC_SQR3_SQ2 : u32 = 1 << 5;
pub const ADC_SQR3_SQ3 : u32 = 1 << 10;
pub const ADC_SQR3_SQ4_0 : u32 = 1 << 15;
pub const ADC_SQR3_SQ4 : u32 = 1 << 16;
pub const ADC_SQR3_SQ5 : u32 = 1 << 20;
pub const ADC_SQR3_SQ6 : u32 = 1 << 25;
pub const ADC_JSQR_JSQ1 : u32 = 1 << 0;
pub const ADC_JSQR_JSQ2 : u32 = 1 << 5;
pub const ADC_JSQR_JSQ3 : u32 = 1 << 10;
pub const ADC_JSQR_JSQ4_0 : u32 = 1 << 15;
pub const ADC_JSQR_JSQ4 : u32 = 1 << 16;
pub const ADC_JSQR_JL : u32 = 1 << 20;
pub const ADC_JDR1_JDATA : u32 = 1 << 0;
pub const ADC_DR_DATA : u32 = 1 << 0;
        
pub fn adc1_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc1_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC1_ADR + ADC1_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc2_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc2_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC2_ADR + ADC2_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc3_sr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_cr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_CR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_cr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_CR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_smpr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SMPR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_smpr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SMPR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_jofr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_JOFR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_htr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_HTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_ltr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_LTR_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr1_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR1_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr2_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR2_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_sqr3_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_SQR3_OFFSET) as *mut u32, value)
    };
}
pub fn adc3_jsqr_write(value: u32) {
    unsafe {
        write_volatile( (ADC3_ADR + ADC3_JSQR_OFFSET) as *mut u32, value)
    };
}


    
        
pub fn adc1_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SR_OFFSET) as *mut u32)
    }
}
pub fn adc1_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc1_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc1_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc1_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc1_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc1_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC1_ADR + ADC1_DR_OFFSET) as *mut u32)
    }
}
    
        
pub fn adc2_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SR_OFFSET) as *mut u32)
    }
}
pub fn adc2_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc2_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc2_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc2_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc2_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc2_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC2_ADR + ADC2_DR_OFFSET) as *mut u32)
    }
}
    
        
pub fn adc3_sr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SR_OFFSET) as *mut u32)
    }
}
pub fn adc3_cr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_CR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_cr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_CR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_smpr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SMPR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_smpr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SMPR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_jofr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JOFR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_htr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_HTR_OFFSET) as *mut u32)
    }
}
pub fn adc3_ltr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_LTR_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr2_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR2_OFFSET) as *mut u32)
    }
}
pub fn adc3_sqr3_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_SQR3_OFFSET) as *mut u32)
    }
}
pub fn adc3_jsqr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JSQR_OFFSET) as *mut u32)
    }
}
pub fn adc3_jdr1_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_JDR1_OFFSET) as *mut u32)
    }
}
pub fn adc3_dr_read() -> u32 {
    unsafe {
        read_volatile( (ADC3_ADR + ADC3_DR_OFFSET) as *mut u32)
    }
}
    
        
pub fn adc1_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_sr_write(rep_bits(adc1_sr_read(), position, size, value));
}
pub fn adc1_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_cr1_write(rep_bits(adc1_cr1_read(), position, size, value));
}
pub fn adc1_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_cr2_write(rep_bits(adc1_cr2_read(), position, size, value));
}
pub fn adc1_smpr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_smpr1_write(rep_bits(adc1_smpr1_read(), position, size, value));
}
pub fn adc1_smpr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_smpr2_write(rep_bits(adc1_smpr2_read(), position, size, value));
}
pub fn adc1_jofr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_jofr1_write(rep_bits(adc1_jofr1_read(), position, size, value));
}
pub fn adc1_htr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_htr_write(rep_bits(adc1_htr_read(), position, size, value));
}
pub fn adc1_ltr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_ltr_write(rep_bits(adc1_ltr_read(), position, size, value));
}
pub fn adc1_sqr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_sqr1_write(rep_bits(adc1_sqr1_read(), position, size, value));
}
pub fn adc1_sqr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_sqr2_write(rep_bits(adc1_sqr2_read(), position, size, value));
}
pub fn adc1_sqr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_sqr3_write(rep_bits(adc1_sqr3_read(), position, size, value));
}
pub fn adc1_jsqr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc1_jsqr_write(rep_bits(adc1_jsqr_read(), position, size, value));
}


    
        
pub fn adc2_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_sr_write(rep_bits(adc2_sr_read(), position, size, value));
}
pub fn adc2_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_cr1_write(rep_bits(adc2_cr1_read(), position, size, value));
}
pub fn adc2_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_cr2_write(rep_bits(adc2_cr2_read(), position, size, value));
}
pub fn adc2_smpr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_smpr1_write(rep_bits(adc2_smpr1_read(), position, size, value));
}
pub fn adc2_smpr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_smpr2_write(rep_bits(adc2_smpr2_read(), position, size, value));
}
pub fn adc2_jofr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_jofr1_write(rep_bits(adc2_jofr1_read(), position, size, value));
}
pub fn adc2_htr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_htr_write(rep_bits(adc2_htr_read(), position, size, value));
}
pub fn adc2_ltr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_ltr_write(rep_bits(adc2_ltr_read(), position, size, value));
}
pub fn adc2_sqr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_sqr1_write(rep_bits(adc2_sqr1_read(), position, size, value));
}
pub fn adc2_sqr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_sqr2_write(rep_bits(adc2_sqr2_read(), position, size, value));
}
pub fn adc2_sqr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_sqr3_write(rep_bits(adc2_sqr3_read(), position, size, value));
}
pub fn adc2_jsqr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc2_jsqr_write(rep_bits(adc2_jsqr_read(), position, size, value));
}


    
        
pub fn adc3_sr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_sr_write(rep_bits(adc3_sr_read(), position, size, value));
}
pub fn adc3_cr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_cr1_write(rep_bits(adc3_cr1_read(), position, size, value));
}
pub fn adc3_cr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_cr2_write(rep_bits(adc3_cr2_read(), position, size, value));
}
pub fn adc3_smpr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_smpr1_write(rep_bits(adc3_smpr1_read(), position, size, value));
}
pub fn adc3_smpr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_smpr2_write(rep_bits(adc3_smpr2_read(), position, size, value));
}
pub fn adc3_jofr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_jofr1_write(rep_bits(adc3_jofr1_read(), position, size, value));
}
pub fn adc3_htr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_htr_write(rep_bits(adc3_htr_read(), position, size, value));
}
pub fn adc3_ltr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_ltr_write(rep_bits(adc3_ltr_read(), position, size, value));
}
pub fn adc3_sqr1_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_sqr1_write(rep_bits(adc3_sqr1_read(), position, size, value));
}
pub fn adc3_sqr2_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_sqr2_write(rep_bits(adc3_sqr2_read(), position, size, value));
}
pub fn adc3_sqr3_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_sqr3_write(rep_bits(adc3_sqr3_read(), position, size, value));
}
pub fn adc3_jsqr_set(position: u32, value: u32) {
    let size = 32 - value.leading_zeros();
    adc3_jsqr_write(rep_bits(adc3_jsqr_read(), position, size, value));
}


    
        
pub fn adc1_sr_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_sr_write(adc1_sr_read() | value),
        31 => adc1_sr_write(adc1_sr_read() & value),
        _ => (),
    }


}
pub fn adc1_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_cr1_write(adc1_cr1_read() | value),
        31 => adc1_cr1_write(adc1_cr1_read() & value),
        _ => (),
    }


}
pub fn adc1_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_cr2_write(adc1_cr2_read() | value),
        31 => adc1_cr2_write(adc1_cr2_read() & value),
        _ => (),
    }


}
pub fn adc1_smpr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_smpr1_write(adc1_smpr1_read() | value),
        31 => adc1_smpr1_write(adc1_smpr1_read() & value),
        _ => (),
    }


}
pub fn adc1_smpr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_smpr2_write(adc1_smpr2_read() | value),
        31 => adc1_smpr2_write(adc1_smpr2_read() & value),
        _ => (),
    }


}
pub fn adc1_jofr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_jofr1_write(adc1_jofr1_read() | value),
        31 => adc1_jofr1_write(adc1_jofr1_read() & value),
        _ => (),
    }


}
pub fn adc1_htr_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_htr_write(adc1_htr_read() | value),
        31 => adc1_htr_write(adc1_htr_read() & value),
        _ => (),
    }


}
pub fn adc1_ltr_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_ltr_write(adc1_ltr_read() | value),
        31 => adc1_ltr_write(adc1_ltr_read() & value),
        _ => (),
    }


}
pub fn adc1_sqr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_sqr1_write(adc1_sqr1_read() | value),
        31 => adc1_sqr1_write(adc1_sqr1_read() & value),
        _ => (),
    }


}
pub fn adc1_sqr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_sqr2_write(adc1_sqr2_read() | value),
        31 => adc1_sqr2_write(adc1_sqr2_read() & value),
        _ => (),
    }


}
pub fn adc1_sqr3_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_sqr3_write(adc1_sqr3_read() | value),
        31 => adc1_sqr3_write(adc1_sqr3_read() & value),
        _ => (),
    }


}
pub fn adc1_jsqr_seti(value: u32) {
    match value.count_ones() {
        1 => adc1_jsqr_write(adc1_jsqr_read() | value),
        31 => adc1_jsqr_write(adc1_jsqr_read() & value),
        _ => (),
    }


}


    
        
pub fn adc2_sr_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_sr_write(adc2_sr_read() | value),
        31 => adc2_sr_write(adc2_sr_read() & value),
        _ => (),
    }


}
pub fn adc2_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_cr1_write(adc2_cr1_read() | value),
        31 => adc2_cr1_write(adc2_cr1_read() & value),
        _ => (),
    }


}
pub fn adc2_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_cr2_write(adc2_cr2_read() | value),
        31 => adc2_cr2_write(adc2_cr2_read() & value),
        _ => (),
    }


}
pub fn adc2_smpr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_smpr1_write(adc2_smpr1_read() | value),
        31 => adc2_smpr1_write(adc2_smpr1_read() & value),
        _ => (),
    }


}
pub fn adc2_smpr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_smpr2_write(adc2_smpr2_read() | value),
        31 => adc2_smpr2_write(adc2_smpr2_read() & value),
        _ => (),
    }


}
pub fn adc2_jofr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_jofr1_write(adc2_jofr1_read() | value),
        31 => adc2_jofr1_write(adc2_jofr1_read() & value),
        _ => (),
    }


}
pub fn adc2_htr_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_htr_write(adc2_htr_read() | value),
        31 => adc2_htr_write(adc2_htr_read() & value),
        _ => (),
    }


}
pub fn adc2_ltr_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_ltr_write(adc2_ltr_read() | value),
        31 => adc2_ltr_write(adc2_ltr_read() & value),
        _ => (),
    }


}
pub fn adc2_sqr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_sqr1_write(adc2_sqr1_read() | value),
        31 => adc2_sqr1_write(adc2_sqr1_read() & value),
        _ => (),
    }


}
pub fn adc2_sqr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_sqr2_write(adc2_sqr2_read() | value),
        31 => adc2_sqr2_write(adc2_sqr2_read() & value),
        _ => (),
    }


}
pub fn adc2_sqr3_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_sqr3_write(adc2_sqr3_read() | value),
        31 => adc2_sqr3_write(adc2_sqr3_read() & value),
        _ => (),
    }


}
pub fn adc2_jsqr_seti(value: u32) {
    match value.count_ones() {
        1 => adc2_jsqr_write(adc2_jsqr_read() | value),
        31 => adc2_jsqr_write(adc2_jsqr_read() & value),
        _ => (),
    }


}


    
        
pub fn adc3_sr_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_sr_write(adc3_sr_read() | value),
        31 => adc3_sr_write(adc3_sr_read() & value),
        _ => (),
    }


}
pub fn adc3_cr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_cr1_write(adc3_cr1_read() | value),
        31 => adc3_cr1_write(adc3_cr1_read() & value),
        _ => (),
    }


}
pub fn adc3_cr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_cr2_write(adc3_cr2_read() | value),
        31 => adc3_cr2_write(adc3_cr2_read() & value),
        _ => (),
    }


}
pub fn adc3_smpr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_smpr1_write(adc3_smpr1_read() | value),
        31 => adc3_smpr1_write(adc3_smpr1_read() & value),
        _ => (),
    }


}
pub fn adc3_smpr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_smpr2_write(adc3_smpr2_read() | value),
        31 => adc3_smpr2_write(adc3_smpr2_read() & value),
        _ => (),
    }


}
pub fn adc3_jofr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_jofr1_write(adc3_jofr1_read() | value),
        31 => adc3_jofr1_write(adc3_jofr1_read() & value),
        _ => (),
    }


}
pub fn adc3_htr_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_htr_write(adc3_htr_read() | value),
        31 => adc3_htr_write(adc3_htr_read() & value),
        _ => (),
    }


}
pub fn adc3_ltr_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_ltr_write(adc3_ltr_read() | value),
        31 => adc3_ltr_write(adc3_ltr_read() & value),
        _ => (),
    }


}
pub fn adc3_sqr1_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_sqr1_write(adc3_sqr1_read() | value),
        31 => adc3_sqr1_write(adc3_sqr1_read() & value),
        _ => (),
    }


}
pub fn adc3_sqr2_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_sqr2_write(adc3_sqr2_read() | value),
        31 => adc3_sqr2_write(adc3_sqr2_read() & value),
        _ => (),
    }


}
pub fn adc3_sqr3_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_sqr3_write(adc3_sqr3_read() | value),
        31 => adc3_sqr3_write(adc3_sqr3_read() & value),
        _ => (),
    }


}
pub fn adc3_jsqr_seti(value: u32) {
    match value.count_ones() {
        1 => adc3_jsqr_write(adc3_jsqr_read() | value),
        31 => adc3_jsqr_write(adc3_jsqr_read() & value),
        _ => (),
    }


}


    