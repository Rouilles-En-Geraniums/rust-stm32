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

const GPIOA_ADR : u32 = 0x40020000;
const GPIOB_ADR : u32 = 0x40020400;
const GPIOC_ADR : u32 = 0x40020800;
const GPIOD_ADR : u32 = 0x40020C00;
const GPIOE_ADR : u32 = 0x40021000;
const GPIOF_ADR : u32 = 0x40021400;
const GPIOG_ADR : u32 = 0x40021800;
const GPIOH_ADR : u32 = 0x40021C00;
const GPIOI_ADR : u32 = 0x40022000;
const GPIOJ_ADR : u32 = 0x40022400;
const GPIOK_ADR : u32 = 0x40022800;
        
const GPIOA_MODER_OFFSET : u32 = 0x00;
const GPIOA_OTYPER_OFFSET : u32 = 0x04;
const GPIOA_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOA_PUPDR_OFFSET : u32 = 0x0c;
const GPIOA_IDR_OFFSET : u32 = 0x10;
const GPIOA_ODR_OFFSET : u32 = 0x14;
const GPIOA_BSRR_OFFSET : u32 = 0x18;
const GPIOA_LCKR_OFFSET : u32 = 0x1c;
const GPIOA_AFRL_OFFSET : u32 = 0x20;
const GPIOA_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOB_MODER_OFFSET : u32 = 0x00;
const GPIOB_OTYPER_OFFSET : u32 = 0x04;
const GPIOB_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOB_PUPDR_OFFSET : u32 = 0x0c;
const GPIOB_IDR_OFFSET : u32 = 0x10;
const GPIOB_ODR_OFFSET : u32 = 0x14;
const GPIOB_BSRR_OFFSET : u32 = 0x18;
const GPIOB_LCKR_OFFSET : u32 = 0x1c;
const GPIOB_AFRL_OFFSET : u32 = 0x20;
const GPIOB_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOC_MODER_OFFSET : u32 = 0x00;
const GPIOC_OTYPER_OFFSET : u32 = 0x04;
const GPIOC_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOC_PUPDR_OFFSET : u32 = 0x0c;
const GPIOC_IDR_OFFSET : u32 = 0x10;
const GPIOC_ODR_OFFSET : u32 = 0x14;
const GPIOC_BSRR_OFFSET : u32 = 0x18;
const GPIOC_LCKR_OFFSET : u32 = 0x1c;
const GPIOC_AFRL_OFFSET : u32 = 0x20;
const GPIOC_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOD_MODER_OFFSET : u32 = 0x00;
const GPIOD_OTYPER_OFFSET : u32 = 0x04;
const GPIOD_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOD_PUPDR_OFFSET : u32 = 0x0c;
const GPIOD_IDR_OFFSET : u32 = 0x10;
const GPIOD_ODR_OFFSET : u32 = 0x14;
const GPIOD_BSRR_OFFSET : u32 = 0x18;
const GPIOD_LCKR_OFFSET : u32 = 0x1c;
const GPIOD_AFRL_OFFSET : u32 = 0x20;
const GPIOD_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOE_MODER_OFFSET : u32 = 0x00;
const GPIOE_OTYPER_OFFSET : u32 = 0x04;
const GPIOE_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOE_PUPDR_OFFSET : u32 = 0x0c;
const GPIOE_IDR_OFFSET : u32 = 0x10;
const GPIOE_ODR_OFFSET : u32 = 0x14;
const GPIOE_BSRR_OFFSET : u32 = 0x18;
const GPIOE_LCKR_OFFSET : u32 = 0x1c;
const GPIOE_AFRL_OFFSET : u32 = 0x20;
const GPIOE_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOF_MODER_OFFSET : u32 = 0x00;
const GPIOF_OTYPER_OFFSET : u32 = 0x04;
const GPIOF_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOF_PUPDR_OFFSET : u32 = 0x0c;
const GPIOF_IDR_OFFSET : u32 = 0x10;
const GPIOF_ODR_OFFSET : u32 = 0x14;
const GPIOF_BSRR_OFFSET : u32 = 0x18;
const GPIOF_LCKR_OFFSET : u32 = 0x1c;
const GPIOF_AFRL_OFFSET : u32 = 0x20;
const GPIOF_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOG_MODER_OFFSET : u32 = 0x00;
const GPIOG_OTYPER_OFFSET : u32 = 0x04;
const GPIOG_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOG_PUPDR_OFFSET : u32 = 0x0c;
const GPIOG_IDR_OFFSET : u32 = 0x10;
const GPIOG_ODR_OFFSET : u32 = 0x14;
const GPIOG_BSRR_OFFSET : u32 = 0x18;
const GPIOG_LCKR_OFFSET : u32 = 0x1c;
const GPIOG_AFRL_OFFSET : u32 = 0x20;
const GPIOG_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOH_MODER_OFFSET : u32 = 0x00;
const GPIOH_OTYPER_OFFSET : u32 = 0x04;
const GPIOH_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOH_PUPDR_OFFSET : u32 = 0x0c;
const GPIOH_IDR_OFFSET : u32 = 0x10;
const GPIOH_ODR_OFFSET : u32 = 0x14;
const GPIOH_BSRR_OFFSET : u32 = 0x18;
const GPIOH_LCKR_OFFSET : u32 = 0x1c;
const GPIOH_AFRL_OFFSET : u32 = 0x20;
const GPIOH_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOI_MODER_OFFSET : u32 = 0x00;
const GPIOI_OTYPER_OFFSET : u32 = 0x04;
const GPIOI_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOI_PUPDR_OFFSET : u32 = 0x0c;
const GPIOI_IDR_OFFSET : u32 = 0x10;
const GPIOI_ODR_OFFSET : u32 = 0x14;
const GPIOI_BSRR_OFFSET : u32 = 0x18;
const GPIOI_LCKR_OFFSET : u32 = 0x1c;
const GPIOI_AFRL_OFFSET : u32 = 0x20;
const GPIOI_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOJ_MODER_OFFSET : u32 = 0x00;
const GPIOJ_OTYPER_OFFSET : u32 = 0x04;
const GPIOJ_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOJ_PUPDR_OFFSET : u32 = 0x0c;
const GPIOJ_IDR_OFFSET : u32 = 0x10;
const GPIOJ_ODR_OFFSET : u32 = 0x14;
const GPIOJ_BSRR_OFFSET : u32 = 0x18;
const GPIOJ_LCKR_OFFSET : u32 = 0x1c;
const GPIOJ_AFRL_OFFSET : u32 = 0x20;
const GPIOJ_AFRH_OFFSET : u32 = 0x24;
    
        
const GPIOK_MODER_OFFSET : u32 = 0x00;
const GPIOK_OTYPER_OFFSET : u32 = 0x04;
const GPIOK_OSPEEDR_OFFSET : u32 = 0x08;
const GPIOK_PUPDR_OFFSET : u32 = 0x0c;
const GPIOK_IDR_OFFSET : u32 = 0x10;
const GPIOK_ODR_OFFSET : u32 = 0x14;
const GPIOK_BSRR_OFFSET : u32 = 0x18;
const GPIOK_LCKR_OFFSET : u32 = 0x1c;
const GPIOK_AFRL_OFFSET : u32 = 0x20;
const GPIOK_AFRH_OFFSET : u32 = 0x24;
    
pub const GPIO_MODER_IN : u32 = 0b00;
pub const GPIO_MODER_OUT : u32 = 0b01;
pub const GPIO_MODER_ALT : u32 = 0b10;
pub const GPIO_MODER_ANA : u32 = 0b11;
pub const GPIO_OSPEEDR_LO : u32 = 0b00;
pub const GPIO_OSPEEDR_ME : u32 = 0b01;
pub const GPIO_OSPEEDR_HI : u32 = 0b10;
pub const GPIO_OSPEEDR_VH : u32 = 0b11;
pub const GPIO_PUPDR_NO : u32 = 0b00;
pub const GPIO_PUPDR_PU : u32 = 0b01;
pub const GPIO_PUPDR_PD : u32 = 0b10;
        
pub fn gpioa_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioa_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOA_ADR + GPIOA_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiob_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiob_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOB_ADR + GPIOB_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioc_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioc_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOC_ADR + GPIOC_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiod_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiod_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOD_ADR + GPIOD_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioe_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioe_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOE_ADR + GPIOE_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiof_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiof_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOF_ADR + GPIOF_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiog_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiog_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOG_ADR + GPIOG_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioh_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioh_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOH_ADR + GPIOH_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioi_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioi_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOI_ADR + GPIOI_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioj_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpioj_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOJ_ADR + GPIOJ_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpiok_moder_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_MODER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_otyper_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_OTYPER_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_ospeedr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_OSPEEDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_pupdr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_PUPDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_idr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_IDR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_odr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_ODR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_bsrr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_BSRR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_lckr_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_LCKR_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_afrl_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_AFRL_OFFSET) as *mut u32, value)
    };
}
pub fn gpiok_afrh_write(value: u32) {
    unsafe {
        write_volatile( (GPIOK_ADR + GPIOK_AFRH_OFFSET) as *mut u32, value)
    };
}
    
        
pub fn gpioa_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioa_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioa_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioa_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioa_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOA_ADR + GPIOA_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiob_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiob_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiob_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiob_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiob_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOB_ADR + GPIOB_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioc_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioc_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioc_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioc_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioc_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOC_ADR + GPIOC_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiod_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiod_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiod_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiod_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiod_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOD_ADR + GPIOD_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioe_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioe_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioe_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioe_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioe_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOE_ADR + GPIOE_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiof_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiof_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiof_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiof_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiof_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOF_ADR + GPIOF_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiog_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiog_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiog_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiog_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiog_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOG_ADR + GPIOG_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioh_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioh_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioh_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioh_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioh_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOH_ADR + GPIOH_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioi_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioi_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioi_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioi_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioi_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOI_ADR + GPIOI_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioj_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpioj_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpioj_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpioj_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpioj_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOJ_ADR + GPIOJ_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpiok_moder_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_MODER_OFFSET) as *mut u32)
    }
}
pub fn gpiok_otyper_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_OTYPER_OFFSET) as *mut u32)
    }
}
pub fn gpiok_ospeedr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_OSPEEDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_pupdr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_PUPDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_idr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_IDR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_odr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_ODR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_bsrr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_BSRR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_lckr_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_LCKR_OFFSET) as *mut u32)
    }
}
pub fn gpiok_afrl_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_AFRL_OFFSET) as *mut u32)
    }
}
pub fn gpiok_afrh_read() -> u32 {
    unsafe {
        read_volatile( (GPIOK_ADR + GPIOK_AFRH_OFFSET) as *mut u32)
    }
}
    
        
pub fn gpioa_moder_set(position: u32, size: u32, value: u32) {
    gpioa_moder_write(rep_bits(gpioa_moder_read(), position, size, value));
}
pub fn gpioa_otyper_set(position: u32, size: u32, value: u32) {
    gpioa_otyper_write(rep_bits(gpioa_otyper_read(), position, size, value));
}
pub fn gpioa_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioa_ospeedr_write(rep_bits(gpioa_ospeedr_read(), position, size, value));
}
pub fn gpioa_pupdr_set(position: u32, size: u32, value: u32) {
    gpioa_pupdr_write(rep_bits(gpioa_pupdr_read(), position, size, value));
}
pub fn gpioa_idr_set(position: u32, size: u32, value: u32) {
    gpioa_idr_write(rep_bits(gpioa_idr_read(), position, size, value));
}
pub fn gpioa_odr_set(position: u32, size: u32, value: u32) {
    gpioa_odr_write(rep_bits(gpioa_odr_read(), position, size, value));
}
pub fn gpioa_bsrr_set(position: u32, size: u32, value: u32) {
    gpioa_bsrr_write(rep_bits(gpioa_bsrr_read(), position, size, value));
}
pub fn gpioa_lckr_set(position: u32, size: u32, value: u32) {
    gpioa_lckr_write(rep_bits(gpioa_lckr_read(), position, size, value));
}
pub fn gpioa_afrl_set(position: u32, size: u32, value: u32) {
    gpioa_afrl_write(rep_bits(gpioa_afrl_read(), position, size, value));
}
pub fn gpioa_afrh_set(position: u32, size: u32, value: u32) {
    gpioa_afrh_write(rep_bits(gpioa_afrh_read(), position, size, value));
}
    
        
pub fn gpiob_moder_set(position: u32, size: u32, value: u32) {
    gpiob_moder_write(rep_bits(gpiob_moder_read(), position, size, value));
}
pub fn gpiob_otyper_set(position: u32, size: u32, value: u32) {
    gpiob_otyper_write(rep_bits(gpiob_otyper_read(), position, size, value));
}
pub fn gpiob_ospeedr_set(position: u32, size: u32, value: u32) {
    gpiob_ospeedr_write(rep_bits(gpiob_ospeedr_read(), position, size, value));
}
pub fn gpiob_pupdr_set(position: u32, size: u32, value: u32) {
    gpiob_pupdr_write(rep_bits(gpiob_pupdr_read(), position, size, value));
}
pub fn gpiob_idr_set(position: u32, size: u32, value: u32) {
    gpiob_idr_write(rep_bits(gpiob_idr_read(), position, size, value));
}
pub fn gpiob_odr_set(position: u32, size: u32, value: u32) {
    gpiob_odr_write(rep_bits(gpiob_odr_read(), position, size, value));
}
pub fn gpiob_bsrr_set(position: u32, size: u32, value: u32) {
    gpiob_bsrr_write(rep_bits(gpiob_bsrr_read(), position, size, value));
}
pub fn gpiob_lckr_set(position: u32, size: u32, value: u32) {
    gpiob_lckr_write(rep_bits(gpiob_lckr_read(), position, size, value));
}
pub fn gpiob_afrl_set(position: u32, size: u32, value: u32) {
    gpiob_afrl_write(rep_bits(gpiob_afrl_read(), position, size, value));
}
pub fn gpiob_afrh_set(position: u32, size: u32, value: u32) {
    gpiob_afrh_write(rep_bits(gpiob_afrh_read(), position, size, value));
}
    
        
pub fn gpioc_moder_set(position: u32, size: u32, value: u32) {
    gpioc_moder_write(rep_bits(gpioc_moder_read(), position, size, value));
}
pub fn gpioc_otyper_set(position: u32, size: u32, value: u32) {
    gpioc_otyper_write(rep_bits(gpioc_otyper_read(), position, size, value));
}
pub fn gpioc_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioc_ospeedr_write(rep_bits(gpioc_ospeedr_read(), position, size, value));
}
pub fn gpioc_pupdr_set(position: u32, size: u32, value: u32) {
    gpioc_pupdr_write(rep_bits(gpioc_pupdr_read(), position, size, value));
}
pub fn gpioc_idr_set(position: u32, size: u32, value: u32) {
    gpioc_idr_write(rep_bits(gpioc_idr_read(), position, size, value));
}
pub fn gpioc_odr_set(position: u32, size: u32, value: u32) {
    gpioc_odr_write(rep_bits(gpioc_odr_read(), position, size, value));
}
pub fn gpioc_bsrr_set(position: u32, size: u32, value: u32) {
    gpioc_bsrr_write(rep_bits(gpioc_bsrr_read(), position, size, value));
}
pub fn gpioc_lckr_set(position: u32, size: u32, value: u32) {
    gpioc_lckr_write(rep_bits(gpioc_lckr_read(), position, size, value));
}
pub fn gpioc_afrl_set(position: u32, size: u32, value: u32) {
    gpioc_afrl_write(rep_bits(gpioc_afrl_read(), position, size, value));
}
pub fn gpioc_afrh_set(position: u32, size: u32, value: u32) {
    gpioc_afrh_write(rep_bits(gpioc_afrh_read(), position, size, value));
}
    
        
pub fn gpiod_moder_set(position: u32, size: u32, value: u32) {
    gpiod_moder_write(rep_bits(gpiod_moder_read(), position, size, value));
}
pub fn gpiod_otyper_set(position: u32, size: u32, value: u32) {
    gpiod_otyper_write(rep_bits(gpiod_otyper_read(), position, size, value));
}
pub fn gpiod_ospeedr_set(position: u32, size: u32, value: u32) {
    gpiod_ospeedr_write(rep_bits(gpiod_ospeedr_read(), position, size, value));
}
pub fn gpiod_pupdr_set(position: u32, size: u32, value: u32) {
    gpiod_pupdr_write(rep_bits(gpiod_pupdr_read(), position, size, value));
}
pub fn gpiod_idr_set(position: u32, size: u32, value: u32) {
    gpiod_idr_write(rep_bits(gpiod_idr_read(), position, size, value));
}
pub fn gpiod_odr_set(position: u32, size: u32, value: u32) {
    gpiod_odr_write(rep_bits(gpiod_odr_read(), position, size, value));
}
pub fn gpiod_bsrr_set(position: u32, size: u32, value: u32) {
    gpiod_bsrr_write(rep_bits(gpiod_bsrr_read(), position, size, value));
}
pub fn gpiod_lckr_set(position: u32, size: u32, value: u32) {
    gpiod_lckr_write(rep_bits(gpiod_lckr_read(), position, size, value));
}
pub fn gpiod_afrl_set(position: u32, size: u32, value: u32) {
    gpiod_afrl_write(rep_bits(gpiod_afrl_read(), position, size, value));
}
pub fn gpiod_afrh_set(position: u32, size: u32, value: u32) {
    gpiod_afrh_write(rep_bits(gpiod_afrh_read(), position, size, value));
}
    
        
pub fn gpioe_moder_set(position: u32, size: u32, value: u32) {
    gpioe_moder_write(rep_bits(gpioe_moder_read(), position, size, value));
}
pub fn gpioe_otyper_set(position: u32, size: u32, value: u32) {
    gpioe_otyper_write(rep_bits(gpioe_otyper_read(), position, size, value));
}
pub fn gpioe_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioe_ospeedr_write(rep_bits(gpioe_ospeedr_read(), position, size, value));
}
pub fn gpioe_pupdr_set(position: u32, size: u32, value: u32) {
    gpioe_pupdr_write(rep_bits(gpioe_pupdr_read(), position, size, value));
}
pub fn gpioe_idr_set(position: u32, size: u32, value: u32) {
    gpioe_idr_write(rep_bits(gpioe_idr_read(), position, size, value));
}
pub fn gpioe_odr_set(position: u32, size: u32, value: u32) {
    gpioe_odr_write(rep_bits(gpioe_odr_read(), position, size, value));
}
pub fn gpioe_bsrr_set(position: u32, size: u32, value: u32) {
    gpioe_bsrr_write(rep_bits(gpioe_bsrr_read(), position, size, value));
}
pub fn gpioe_lckr_set(position: u32, size: u32, value: u32) {
    gpioe_lckr_write(rep_bits(gpioe_lckr_read(), position, size, value));
}
pub fn gpioe_afrl_set(position: u32, size: u32, value: u32) {
    gpioe_afrl_write(rep_bits(gpioe_afrl_read(), position, size, value));
}
pub fn gpioe_afrh_set(position: u32, size: u32, value: u32) {
    gpioe_afrh_write(rep_bits(gpioe_afrh_read(), position, size, value));
}
    
        
pub fn gpiof_moder_set(position: u32, size: u32, value: u32) {
    gpiof_moder_write(rep_bits(gpiof_moder_read(), position, size, value));
}
pub fn gpiof_otyper_set(position: u32, size: u32, value: u32) {
    gpiof_otyper_write(rep_bits(gpiof_otyper_read(), position, size, value));
}
pub fn gpiof_ospeedr_set(position: u32, size: u32, value: u32) {
    gpiof_ospeedr_write(rep_bits(gpiof_ospeedr_read(), position, size, value));
}
pub fn gpiof_pupdr_set(position: u32, size: u32, value: u32) {
    gpiof_pupdr_write(rep_bits(gpiof_pupdr_read(), position, size, value));
}
pub fn gpiof_idr_set(position: u32, size: u32, value: u32) {
    gpiof_idr_write(rep_bits(gpiof_idr_read(), position, size, value));
}
pub fn gpiof_odr_set(position: u32, size: u32, value: u32) {
    gpiof_odr_write(rep_bits(gpiof_odr_read(), position, size, value));
}
pub fn gpiof_bsrr_set(position: u32, size: u32, value: u32) {
    gpiof_bsrr_write(rep_bits(gpiof_bsrr_read(), position, size, value));
}
pub fn gpiof_lckr_set(position: u32, size: u32, value: u32) {
    gpiof_lckr_write(rep_bits(gpiof_lckr_read(), position, size, value));
}
pub fn gpiof_afrl_set(position: u32, size: u32, value: u32) {
    gpiof_afrl_write(rep_bits(gpiof_afrl_read(), position, size, value));
}
pub fn gpiof_afrh_set(position: u32, size: u32, value: u32) {
    gpiof_afrh_write(rep_bits(gpiof_afrh_read(), position, size, value));
}
    
        
pub fn gpiog_moder_set(position: u32, size: u32, value: u32) {
    gpiog_moder_write(rep_bits(gpiog_moder_read(), position, size, value));
}
pub fn gpiog_otyper_set(position: u32, size: u32, value: u32) {
    gpiog_otyper_write(rep_bits(gpiog_otyper_read(), position, size, value));
}
pub fn gpiog_ospeedr_set(position: u32, size: u32, value: u32) {
    gpiog_ospeedr_write(rep_bits(gpiog_ospeedr_read(), position, size, value));
}
pub fn gpiog_pupdr_set(position: u32, size: u32, value: u32) {
    gpiog_pupdr_write(rep_bits(gpiog_pupdr_read(), position, size, value));
}
pub fn gpiog_idr_set(position: u32, size: u32, value: u32) {
    gpiog_idr_write(rep_bits(gpiog_idr_read(), position, size, value));
}
pub fn gpiog_odr_set(position: u32, size: u32, value: u32) {
    gpiog_odr_write(rep_bits(gpiog_odr_read(), position, size, value));
}
pub fn gpiog_bsrr_set(position: u32, size: u32, value: u32) {
    gpiog_bsrr_write(rep_bits(gpiog_bsrr_read(), position, size, value));
}
pub fn gpiog_lckr_set(position: u32, size: u32, value: u32) {
    gpiog_lckr_write(rep_bits(gpiog_lckr_read(), position, size, value));
}
pub fn gpiog_afrl_set(position: u32, size: u32, value: u32) {
    gpiog_afrl_write(rep_bits(gpiog_afrl_read(), position, size, value));
}
pub fn gpiog_afrh_set(position: u32, size: u32, value: u32) {
    gpiog_afrh_write(rep_bits(gpiog_afrh_read(), position, size, value));
}
    
        
pub fn gpioh_moder_set(position: u32, size: u32, value: u32) {
    gpioh_moder_write(rep_bits(gpioh_moder_read(), position, size, value));
}
pub fn gpioh_otyper_set(position: u32, size: u32, value: u32) {
    gpioh_otyper_write(rep_bits(gpioh_otyper_read(), position, size, value));
}
pub fn gpioh_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioh_ospeedr_write(rep_bits(gpioh_ospeedr_read(), position, size, value));
}
pub fn gpioh_pupdr_set(position: u32, size: u32, value: u32) {
    gpioh_pupdr_write(rep_bits(gpioh_pupdr_read(), position, size, value));
}
pub fn gpioh_idr_set(position: u32, size: u32, value: u32) {
    gpioh_idr_write(rep_bits(gpioh_idr_read(), position, size, value));
}
pub fn gpioh_odr_set(position: u32, size: u32, value: u32) {
    gpioh_odr_write(rep_bits(gpioh_odr_read(), position, size, value));
}
pub fn gpioh_bsrr_set(position: u32, size: u32, value: u32) {
    gpioh_bsrr_write(rep_bits(gpioh_bsrr_read(), position, size, value));
}
pub fn gpioh_lckr_set(position: u32, size: u32, value: u32) {
    gpioh_lckr_write(rep_bits(gpioh_lckr_read(), position, size, value));
}
pub fn gpioh_afrl_set(position: u32, size: u32, value: u32) {
    gpioh_afrl_write(rep_bits(gpioh_afrl_read(), position, size, value));
}
pub fn gpioh_afrh_set(position: u32, size: u32, value: u32) {
    gpioh_afrh_write(rep_bits(gpioh_afrh_read(), position, size, value));
}
    
        
pub fn gpioi_moder_set(position: u32, size: u32, value: u32) {
    gpioi_moder_write(rep_bits(gpioi_moder_read(), position, size, value));
}
pub fn gpioi_otyper_set(position: u32, size: u32, value: u32) {
    gpioi_otyper_write(rep_bits(gpioi_otyper_read(), position, size, value));
}
pub fn gpioi_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioi_ospeedr_write(rep_bits(gpioi_ospeedr_read(), position, size, value));
}
pub fn gpioi_pupdr_set(position: u32, size: u32, value: u32) {
    gpioi_pupdr_write(rep_bits(gpioi_pupdr_read(), position, size, value));
}
pub fn gpioi_idr_set(position: u32, size: u32, value: u32) {
    gpioi_idr_write(rep_bits(gpioi_idr_read(), position, size, value));
}
pub fn gpioi_odr_set(position: u32, size: u32, value: u32) {
    gpioi_odr_write(rep_bits(gpioi_odr_read(), position, size, value));
}
pub fn gpioi_bsrr_set(position: u32, size: u32, value: u32) {
    gpioi_bsrr_write(rep_bits(gpioi_bsrr_read(), position, size, value));
}
pub fn gpioi_lckr_set(position: u32, size: u32, value: u32) {
    gpioi_lckr_write(rep_bits(gpioi_lckr_read(), position, size, value));
}
pub fn gpioi_afrl_set(position: u32, size: u32, value: u32) {
    gpioi_afrl_write(rep_bits(gpioi_afrl_read(), position, size, value));
}
pub fn gpioi_afrh_set(position: u32, size: u32, value: u32) {
    gpioi_afrh_write(rep_bits(gpioi_afrh_read(), position, size, value));
}
    
        
pub fn gpioj_moder_set(position: u32, size: u32, value: u32) {
    gpioj_moder_write(rep_bits(gpioj_moder_read(), position, size, value));
}
pub fn gpioj_otyper_set(position: u32, size: u32, value: u32) {
    gpioj_otyper_write(rep_bits(gpioj_otyper_read(), position, size, value));
}
pub fn gpioj_ospeedr_set(position: u32, size: u32, value: u32) {
    gpioj_ospeedr_write(rep_bits(gpioj_ospeedr_read(), position, size, value));
}
pub fn gpioj_pupdr_set(position: u32, size: u32, value: u32) {
    gpioj_pupdr_write(rep_bits(gpioj_pupdr_read(), position, size, value));
}
pub fn gpioj_idr_set(position: u32, size: u32, value: u32) {
    gpioj_idr_write(rep_bits(gpioj_idr_read(), position, size, value));
}
pub fn gpioj_odr_set(position: u32, size: u32, value: u32) {
    gpioj_odr_write(rep_bits(gpioj_odr_read(), position, size, value));
}
pub fn gpioj_bsrr_set(position: u32, size: u32, value: u32) {
    gpioj_bsrr_write(rep_bits(gpioj_bsrr_read(), position, size, value));
}
pub fn gpioj_lckr_set(position: u32, size: u32, value: u32) {
    gpioj_lckr_write(rep_bits(gpioj_lckr_read(), position, size, value));
}
pub fn gpioj_afrl_set(position: u32, size: u32, value: u32) {
    gpioj_afrl_write(rep_bits(gpioj_afrl_read(), position, size, value));
}
pub fn gpioj_afrh_set(position: u32, size: u32, value: u32) {
    gpioj_afrh_write(rep_bits(gpioj_afrh_read(), position, size, value));
}
    
        
pub fn gpiok_moder_set(position: u32, size: u32, value: u32) {
    gpiok_moder_write(rep_bits(gpiok_moder_read(), position, size, value));
}
pub fn gpiok_otyper_set(position: u32, size: u32, value: u32) {
    gpiok_otyper_write(rep_bits(gpiok_otyper_read(), position, size, value));
}
pub fn gpiok_ospeedr_set(position: u32, size: u32, value: u32) {
    gpiok_ospeedr_write(rep_bits(gpiok_ospeedr_read(), position, size, value));
}
pub fn gpiok_pupdr_set(position: u32, size: u32, value: u32) {
    gpiok_pupdr_write(rep_bits(gpiok_pupdr_read(), position, size, value));
}
pub fn gpiok_idr_set(position: u32, size: u32, value: u32) {
    gpiok_idr_write(rep_bits(gpiok_idr_read(), position, size, value));
}
pub fn gpiok_odr_set(position: u32, size: u32, value: u32) {
    gpiok_odr_write(rep_bits(gpiok_odr_read(), position, size, value));
}
pub fn gpiok_bsrr_set(position: u32, size: u32, value: u32) {
    gpiok_bsrr_write(rep_bits(gpiok_bsrr_read(), position, size, value));
}
pub fn gpiok_lckr_set(position: u32, size: u32, value: u32) {
    gpiok_lckr_write(rep_bits(gpiok_lckr_read(), position, size, value));
}
pub fn gpiok_afrl_set(position: u32, size: u32, value: u32) {
    gpiok_afrl_write(rep_bits(gpiok_afrl_read(), position, size, value));
}
pub fn gpiok_afrh_set(position: u32, size: u32, value: u32) {
    gpiok_afrh_write(rep_bits(gpiok_afrh_read(), position, size, value));
}
    
        
pub fn gpioa_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_moder_write(gpioa_moder_read() | value),
        31 => gpioa_moder_write(gpioa_moder_read() & value),
        _ => (),}
}
pub fn gpioa_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_otyper_write(gpioa_otyper_read() | value),
        31 => gpioa_otyper_write(gpioa_otyper_read() & value),
        _ => (),}
}
pub fn gpioa_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_ospeedr_write(gpioa_ospeedr_read() | value),
        31 => gpioa_ospeedr_write(gpioa_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioa_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_pupdr_write(gpioa_pupdr_read() | value),
        31 => gpioa_pupdr_write(gpioa_pupdr_read() & value),
        _ => (),}
}
pub fn gpioa_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_idr_write(gpioa_idr_read() | value),
        31 => gpioa_idr_write(gpioa_idr_read() & value),
        _ => (),}
}
pub fn gpioa_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_odr_write(gpioa_odr_read() | value),
        31 => gpioa_odr_write(gpioa_odr_read() & value),
        _ => (),}
}
pub fn gpioa_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_bsrr_write(gpioa_bsrr_read() | value),
        31 => gpioa_bsrr_write(gpioa_bsrr_read() & value),
        _ => (),}
}
pub fn gpioa_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_lckr_write(gpioa_lckr_read() | value),
        31 => gpioa_lckr_write(gpioa_lckr_read() & value),
        _ => (),}
}
pub fn gpioa_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_afrl_write(gpioa_afrl_read() | value),
        31 => gpioa_afrl_write(gpioa_afrl_read() & value),
        _ => (),}
}
pub fn gpioa_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioa_afrh_write(gpioa_afrh_read() | value),
        31 => gpioa_afrh_write(gpioa_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpiob_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_moder_write(gpiob_moder_read() | value),
        31 => gpiob_moder_write(gpiob_moder_read() & value),
        _ => (),}
}
pub fn gpiob_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_otyper_write(gpiob_otyper_read() | value),
        31 => gpiob_otyper_write(gpiob_otyper_read() & value),
        _ => (),}
}
pub fn gpiob_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_ospeedr_write(gpiob_ospeedr_read() | value),
        31 => gpiob_ospeedr_write(gpiob_ospeedr_read() & value),
        _ => (),}
}
pub fn gpiob_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_pupdr_write(gpiob_pupdr_read() | value),
        31 => gpiob_pupdr_write(gpiob_pupdr_read() & value),
        _ => (),}
}
pub fn gpiob_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_idr_write(gpiob_idr_read() | value),
        31 => gpiob_idr_write(gpiob_idr_read() & value),
        _ => (),}
}
pub fn gpiob_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_odr_write(gpiob_odr_read() | value),
        31 => gpiob_odr_write(gpiob_odr_read() & value),
        _ => (),}
}
pub fn gpiob_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_bsrr_write(gpiob_bsrr_read() | value),
        31 => gpiob_bsrr_write(gpiob_bsrr_read() & value),
        _ => (),}
}
pub fn gpiob_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_lckr_write(gpiob_lckr_read() | value),
        31 => gpiob_lckr_write(gpiob_lckr_read() & value),
        _ => (),}
}
pub fn gpiob_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_afrl_write(gpiob_afrl_read() | value),
        31 => gpiob_afrl_write(gpiob_afrl_read() & value),
        _ => (),}
}
pub fn gpiob_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpiob_afrh_write(gpiob_afrh_read() | value),
        31 => gpiob_afrh_write(gpiob_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpioc_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_moder_write(gpioc_moder_read() | value),
        31 => gpioc_moder_write(gpioc_moder_read() & value),
        _ => (),}
}
pub fn gpioc_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_otyper_write(gpioc_otyper_read() | value),
        31 => gpioc_otyper_write(gpioc_otyper_read() & value),
        _ => (),}
}
pub fn gpioc_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_ospeedr_write(gpioc_ospeedr_read() | value),
        31 => gpioc_ospeedr_write(gpioc_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioc_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_pupdr_write(gpioc_pupdr_read() | value),
        31 => gpioc_pupdr_write(gpioc_pupdr_read() & value),
        _ => (),}
}
pub fn gpioc_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_idr_write(gpioc_idr_read() | value),
        31 => gpioc_idr_write(gpioc_idr_read() & value),
        _ => (),}
}
pub fn gpioc_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_odr_write(gpioc_odr_read() | value),
        31 => gpioc_odr_write(gpioc_odr_read() & value),
        _ => (),}
}
pub fn gpioc_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_bsrr_write(gpioc_bsrr_read() | value),
        31 => gpioc_bsrr_write(gpioc_bsrr_read() & value),
        _ => (),}
}
pub fn gpioc_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_lckr_write(gpioc_lckr_read() | value),
        31 => gpioc_lckr_write(gpioc_lckr_read() & value),
        _ => (),}
}
pub fn gpioc_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_afrl_write(gpioc_afrl_read() | value),
        31 => gpioc_afrl_write(gpioc_afrl_read() & value),
        _ => (),}
}
pub fn gpioc_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioc_afrh_write(gpioc_afrh_read() | value),
        31 => gpioc_afrh_write(gpioc_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpiod_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_moder_write(gpiod_moder_read() | value),
        31 => gpiod_moder_write(gpiod_moder_read() & value),
        _ => (),}
}
pub fn gpiod_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_otyper_write(gpiod_otyper_read() | value),
        31 => gpiod_otyper_write(gpiod_otyper_read() & value),
        _ => (),}
}
pub fn gpiod_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_ospeedr_write(gpiod_ospeedr_read() | value),
        31 => gpiod_ospeedr_write(gpiod_ospeedr_read() & value),
        _ => (),}
}
pub fn gpiod_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_pupdr_write(gpiod_pupdr_read() | value),
        31 => gpiod_pupdr_write(gpiod_pupdr_read() & value),
        _ => (),}
}
pub fn gpiod_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_idr_write(gpiod_idr_read() | value),
        31 => gpiod_idr_write(gpiod_idr_read() & value),
        _ => (),}
}
pub fn gpiod_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_odr_write(gpiod_odr_read() | value),
        31 => gpiod_odr_write(gpiod_odr_read() & value),
        _ => (),}
}
pub fn gpiod_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_bsrr_write(gpiod_bsrr_read() | value),
        31 => gpiod_bsrr_write(gpiod_bsrr_read() & value),
        _ => (),}
}
pub fn gpiod_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_lckr_write(gpiod_lckr_read() | value),
        31 => gpiod_lckr_write(gpiod_lckr_read() & value),
        _ => (),}
}
pub fn gpiod_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_afrl_write(gpiod_afrl_read() | value),
        31 => gpiod_afrl_write(gpiod_afrl_read() & value),
        _ => (),}
}
pub fn gpiod_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpiod_afrh_write(gpiod_afrh_read() | value),
        31 => gpiod_afrh_write(gpiod_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpioe_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_moder_write(gpioe_moder_read() | value),
        31 => gpioe_moder_write(gpioe_moder_read() & value),
        _ => (),}
}
pub fn gpioe_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_otyper_write(gpioe_otyper_read() | value),
        31 => gpioe_otyper_write(gpioe_otyper_read() & value),
        _ => (),}
}
pub fn gpioe_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_ospeedr_write(gpioe_ospeedr_read() | value),
        31 => gpioe_ospeedr_write(gpioe_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioe_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_pupdr_write(gpioe_pupdr_read() | value),
        31 => gpioe_pupdr_write(gpioe_pupdr_read() & value),
        _ => (),}
}
pub fn gpioe_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_idr_write(gpioe_idr_read() | value),
        31 => gpioe_idr_write(gpioe_idr_read() & value),
        _ => (),}
}
pub fn gpioe_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_odr_write(gpioe_odr_read() | value),
        31 => gpioe_odr_write(gpioe_odr_read() & value),
        _ => (),}
}
pub fn gpioe_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_bsrr_write(gpioe_bsrr_read() | value),
        31 => gpioe_bsrr_write(gpioe_bsrr_read() & value),
        _ => (),}
}
pub fn gpioe_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_lckr_write(gpioe_lckr_read() | value),
        31 => gpioe_lckr_write(gpioe_lckr_read() & value),
        _ => (),}
}
pub fn gpioe_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_afrl_write(gpioe_afrl_read() | value),
        31 => gpioe_afrl_write(gpioe_afrl_read() & value),
        _ => (),}
}
pub fn gpioe_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioe_afrh_write(gpioe_afrh_read() | value),
        31 => gpioe_afrh_write(gpioe_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpiof_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_moder_write(gpiof_moder_read() | value),
        31 => gpiof_moder_write(gpiof_moder_read() & value),
        _ => (),}
}
pub fn gpiof_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_otyper_write(gpiof_otyper_read() | value),
        31 => gpiof_otyper_write(gpiof_otyper_read() & value),
        _ => (),}
}
pub fn gpiof_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_ospeedr_write(gpiof_ospeedr_read() | value),
        31 => gpiof_ospeedr_write(gpiof_ospeedr_read() & value),
        _ => (),}
}
pub fn gpiof_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_pupdr_write(gpiof_pupdr_read() | value),
        31 => gpiof_pupdr_write(gpiof_pupdr_read() & value),
        _ => (),}
}
pub fn gpiof_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_idr_write(gpiof_idr_read() | value),
        31 => gpiof_idr_write(gpiof_idr_read() & value),
        _ => (),}
}
pub fn gpiof_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_odr_write(gpiof_odr_read() | value),
        31 => gpiof_odr_write(gpiof_odr_read() & value),
        _ => (),}
}
pub fn gpiof_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_bsrr_write(gpiof_bsrr_read() | value),
        31 => gpiof_bsrr_write(gpiof_bsrr_read() & value),
        _ => (),}
}
pub fn gpiof_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_lckr_write(gpiof_lckr_read() | value),
        31 => gpiof_lckr_write(gpiof_lckr_read() & value),
        _ => (),}
}
pub fn gpiof_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_afrl_write(gpiof_afrl_read() | value),
        31 => gpiof_afrl_write(gpiof_afrl_read() & value),
        _ => (),}
}
pub fn gpiof_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpiof_afrh_write(gpiof_afrh_read() | value),
        31 => gpiof_afrh_write(gpiof_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpiog_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_moder_write(gpiog_moder_read() | value),
        31 => gpiog_moder_write(gpiog_moder_read() & value),
        _ => (),}
}
pub fn gpiog_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_otyper_write(gpiog_otyper_read() | value),
        31 => gpiog_otyper_write(gpiog_otyper_read() & value),
        _ => (),}
}
pub fn gpiog_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_ospeedr_write(gpiog_ospeedr_read() | value),
        31 => gpiog_ospeedr_write(gpiog_ospeedr_read() & value),
        _ => (),}
}
pub fn gpiog_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_pupdr_write(gpiog_pupdr_read() | value),
        31 => gpiog_pupdr_write(gpiog_pupdr_read() & value),
        _ => (),}
}
pub fn gpiog_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_idr_write(gpiog_idr_read() | value),
        31 => gpiog_idr_write(gpiog_idr_read() & value),
        _ => (),}
}
pub fn gpiog_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_odr_write(gpiog_odr_read() | value),
        31 => gpiog_odr_write(gpiog_odr_read() & value),
        _ => (),}
}
pub fn gpiog_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_bsrr_write(gpiog_bsrr_read() | value),
        31 => gpiog_bsrr_write(gpiog_bsrr_read() & value),
        _ => (),}
}
pub fn gpiog_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_lckr_write(gpiog_lckr_read() | value),
        31 => gpiog_lckr_write(gpiog_lckr_read() & value),
        _ => (),}
}
pub fn gpiog_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_afrl_write(gpiog_afrl_read() | value),
        31 => gpiog_afrl_write(gpiog_afrl_read() & value),
        _ => (),}
}
pub fn gpiog_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpiog_afrh_write(gpiog_afrh_read() | value),
        31 => gpiog_afrh_write(gpiog_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpioh_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_moder_write(gpioh_moder_read() | value),
        31 => gpioh_moder_write(gpioh_moder_read() & value),
        _ => (),}
}
pub fn gpioh_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_otyper_write(gpioh_otyper_read() | value),
        31 => gpioh_otyper_write(gpioh_otyper_read() & value),
        _ => (),}
}
pub fn gpioh_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_ospeedr_write(gpioh_ospeedr_read() | value),
        31 => gpioh_ospeedr_write(gpioh_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioh_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_pupdr_write(gpioh_pupdr_read() | value),
        31 => gpioh_pupdr_write(gpioh_pupdr_read() & value),
        _ => (),}
}
pub fn gpioh_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_idr_write(gpioh_idr_read() | value),
        31 => gpioh_idr_write(gpioh_idr_read() & value),
        _ => (),}
}
pub fn gpioh_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_odr_write(gpioh_odr_read() | value),
        31 => gpioh_odr_write(gpioh_odr_read() & value),
        _ => (),}
}
pub fn gpioh_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_bsrr_write(gpioh_bsrr_read() | value),
        31 => gpioh_bsrr_write(gpioh_bsrr_read() & value),
        _ => (),}
}
pub fn gpioh_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_lckr_write(gpioh_lckr_read() | value),
        31 => gpioh_lckr_write(gpioh_lckr_read() & value),
        _ => (),}
}
pub fn gpioh_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_afrl_write(gpioh_afrl_read() | value),
        31 => gpioh_afrl_write(gpioh_afrl_read() & value),
        _ => (),}
}
pub fn gpioh_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioh_afrh_write(gpioh_afrh_read() | value),
        31 => gpioh_afrh_write(gpioh_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpioi_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_moder_write(gpioi_moder_read() | value),
        31 => gpioi_moder_write(gpioi_moder_read() & value),
        _ => (),}
}
pub fn gpioi_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_otyper_write(gpioi_otyper_read() | value),
        31 => gpioi_otyper_write(gpioi_otyper_read() & value),
        _ => (),}
}
pub fn gpioi_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_ospeedr_write(gpioi_ospeedr_read() | value),
        31 => gpioi_ospeedr_write(gpioi_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioi_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_pupdr_write(gpioi_pupdr_read() | value),
        31 => gpioi_pupdr_write(gpioi_pupdr_read() & value),
        _ => (),}
}
pub fn gpioi_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_idr_write(gpioi_idr_read() | value),
        31 => gpioi_idr_write(gpioi_idr_read() & value),
        _ => (),}
}
pub fn gpioi_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_odr_write(gpioi_odr_read() | value),
        31 => gpioi_odr_write(gpioi_odr_read() & value),
        _ => (),}
}
pub fn gpioi_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_bsrr_write(gpioi_bsrr_read() | value),
        31 => gpioi_bsrr_write(gpioi_bsrr_read() & value),
        _ => (),}
}
pub fn gpioi_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_lckr_write(gpioi_lckr_read() | value),
        31 => gpioi_lckr_write(gpioi_lckr_read() & value),
        _ => (),}
}
pub fn gpioi_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_afrl_write(gpioi_afrl_read() | value),
        31 => gpioi_afrl_write(gpioi_afrl_read() & value),
        _ => (),}
}
pub fn gpioi_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioi_afrh_write(gpioi_afrh_read() | value),
        31 => gpioi_afrh_write(gpioi_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpioj_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_moder_write(gpioj_moder_read() | value),
        31 => gpioj_moder_write(gpioj_moder_read() & value),
        _ => (),}
}
pub fn gpioj_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_otyper_write(gpioj_otyper_read() | value),
        31 => gpioj_otyper_write(gpioj_otyper_read() & value),
        _ => (),}
}
pub fn gpioj_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_ospeedr_write(gpioj_ospeedr_read() | value),
        31 => gpioj_ospeedr_write(gpioj_ospeedr_read() & value),
        _ => (),}
}
pub fn gpioj_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_pupdr_write(gpioj_pupdr_read() | value),
        31 => gpioj_pupdr_write(gpioj_pupdr_read() & value),
        _ => (),}
}
pub fn gpioj_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_idr_write(gpioj_idr_read() | value),
        31 => gpioj_idr_write(gpioj_idr_read() & value),
        _ => (),}
}
pub fn gpioj_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_odr_write(gpioj_odr_read() | value),
        31 => gpioj_odr_write(gpioj_odr_read() & value),
        _ => (),}
}
pub fn gpioj_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_bsrr_write(gpioj_bsrr_read() | value),
        31 => gpioj_bsrr_write(gpioj_bsrr_read() & value),
        _ => (),}
}
pub fn gpioj_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_lckr_write(gpioj_lckr_read() | value),
        31 => gpioj_lckr_write(gpioj_lckr_read() & value),
        _ => (),}
}
pub fn gpioj_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_afrl_write(gpioj_afrl_read() | value),
        31 => gpioj_afrl_write(gpioj_afrl_read() & value),
        _ => (),}
}
pub fn gpioj_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpioj_afrh_write(gpioj_afrh_read() | value),
        31 => gpioj_afrh_write(gpioj_afrh_read() & value),
        _ => (),}
}
    
        
pub fn gpiok_moder_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_moder_write(gpiok_moder_read() | value),
        31 => gpiok_moder_write(gpiok_moder_read() & value),
        _ => (),}
}
pub fn gpiok_otyper_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_otyper_write(gpiok_otyper_read() | value),
        31 => gpiok_otyper_write(gpiok_otyper_read() & value),
        _ => (),}
}
pub fn gpiok_ospeedr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_ospeedr_write(gpiok_ospeedr_read() | value),
        31 => gpiok_ospeedr_write(gpiok_ospeedr_read() & value),
        _ => (),}
}
pub fn gpiok_pupdr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_pupdr_write(gpiok_pupdr_read() | value),
        31 => gpiok_pupdr_write(gpiok_pupdr_read() & value),
        _ => (),}
}
pub fn gpiok_idr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_idr_write(gpiok_idr_read() | value),
        31 => gpiok_idr_write(gpiok_idr_read() & value),
        _ => (),}
}
pub fn gpiok_odr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_odr_write(gpiok_odr_read() | value),
        31 => gpiok_odr_write(gpiok_odr_read() & value),
        _ => (),}
}
pub fn gpiok_bsrr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_bsrr_write(gpiok_bsrr_read() | value),
        31 => gpiok_bsrr_write(gpiok_bsrr_read() & value),
        _ => (),}
}
pub fn gpiok_lckr_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_lckr_write(gpiok_lckr_read() | value),
        31 => gpiok_lckr_write(gpiok_lckr_read() & value),
        _ => (),}
}
pub fn gpiok_afrl_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_afrl_write(gpiok_afrl_read() | value),
        31 => gpiok_afrl_write(gpiok_afrl_read() & value),
        _ => (),}
}
pub fn gpiok_afrh_seti(value: u32) {
    match value.count_ones() {
        1 => gpiok_afrh_write(gpiok_afrh_read() | value),
        31 => gpiok_afrh_write(gpiok_afrh_read() & value),
        _ => (),}
}
    

/**
 * pin = (GPIO# : char, pin# : u32)
 * mode = HIGH/LOW
 */
pub fn digital_write(pin: (char,u32), mode: u8){
    match pin.0 {
        'A' => {
            match mode {
                HIGH => gpioa_bsrr_write(1 << pin.1),
                LOW => gpioa_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'B' => {
            match mode {
                HIGH => gpiob_bsrr_write(1 << pin.1),
                LOW => gpiob_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'C' => {
            match mode {
                HIGH => gpioc_bsrr_write(1 << pin.1),
                LOW => gpioc_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'D' => {
            match mode {
                HIGH => gpiod_bsrr_write(1 << pin.1),
                LOW => gpiod_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'E' => {
            match mode {
                HIGH => gpioe_bsrr_write(1 << pin.1),
                LOW => gpioe_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'F' => {
            match mode {
                HIGH => gpiof_bsrr_write(1 << pin.1),
                LOW => gpiof_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'G' => {
            match mode {
                HIGH => gpiog_bsrr_write(1 << pin.1),
                LOW => gpiog_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'H' => {
            match mode {
                HIGH => gpioh_bsrr_write(1 << pin.1),
                LOW => gpioh_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'I' => {
            match mode {
                HIGH => gpioi_bsrr_write(1 << pin.1),
                LOW => gpioi_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'J' => {
            match mode {
                HIGH => gpioj_bsrr_write(1 << pin.1),
                LOW => gpioj_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        'K' => {
            match mode {
                HIGH => gpiok_bsrr_write(1 << pin.1),
                LOW => gpiok_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
        _ => (),
    }
}

/**
 * Reads the input of a given pin
 * pin = (GPIO# : char, pin# : u32)
 */
pub fn digital_read(pin: (char,u32)) -> u8 {
    match pin.0 {
        'A' => {
            if (gpioa_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'B' => {
            if (gpiob_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'C' => {
            if (gpioc_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'D' => {
            if (gpiod_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'E' => {
            if (gpioe_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'F' => {
            if (gpiof_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'G' => {
            if (gpiog_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'H' => {
            if (gpioh_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'I' => {
            if (gpioi_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'J' => {
            if (gpioj_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        'K' => {
            if (gpiok_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        },
        _ => 2
    }
}