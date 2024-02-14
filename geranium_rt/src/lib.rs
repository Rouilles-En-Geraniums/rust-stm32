#![no_std]
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


use core::panic::PanicInfo;
use core::ptr;

extern crate core;
pub mod stm32rustlib;
use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::various::*;
use crate::stm32rustlib::syscfg::*;
use crate::stm32rustlib::gdb::*;


pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}


extern "C" {
    static mut _data_flash: u8;
    static mut _data_begin: u8;
    static mut _data_end: u8;
    static mut _bss_begin: u8;
    static mut _bss_end: u8;
    static mut _stack_end: u8;
}


#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: DefaultExceptionHandler },  // 2 - NMI (non maskable interrupt)
    Vector { handler: DefaultExceptionHandler }, // 3 - Hard fault
    Vector { handler: DefaultExceptionHandler }, // 4 - Memmanage fault (MPU violation or access to illegal location)
    Vector { handler: DefaultExceptionHandler }, // 5 - Bus fault (bus error)
    Vector { handler: DefaultExceptionHandler }, // 6 - Usage fault (program error eg acces coprocessor)
    Vector { reserved: 0 }, // 7 - Reserved
    Vector { reserved: 0 }, // 8 - Reserved
    Vector { reserved: 0 }, // 9 - Reserved
    Vector { reserved: 0 }, // 10 - Reserved
    Vector { handler: DefaultExceptionHandler }, // 11 - SVC (Supervisor call)
    Vector { reserved: 0 }, // 12 - Debug monitor (BP, WP, external debug requests)
    Vector { reserved: 0 }, // 13 - Reserved
    Vector { handler: DefaultExceptionHandler }, // 14 - PendSV (pendable service call)
    Vector { handler: DefaultExceptionHandler }, // 15 - SysTick (System tick timer)
];


#[link_section = ".vector_table.custom_exceptions"]
#[no_mangle]
pub static CUSTOM_EXCEPTIONS: [Vector; 44] = [
    Vector { handler: DefaultExceptionHandler }, // 16 
    Vector { handler: DefaultExceptionHandler }, // 17
    Vector { handler: DefaultExceptionHandler }, // 18
    Vector { handler: DefaultExceptionHandler }, // 19
    Vector { handler: DefaultExceptionHandler }, // 21
    Vector { handler: DefaultExceptionHandler }, // 22
    Vector { handler: DefaultExceptionHandler }, // 23
    Vector { handler: DefaultExceptionHandler }, // 24
    Vector { handler: DefaultExceptionHandler }, // 25
    Vector { handler: DefaultExceptionHandler }, // 26
    Vector { handler: DefaultExceptionHandler }, // 27
    Vector { handler: DefaultExceptionHandler }, // 28
    Vector { handler: DefaultExceptionHandler }, // 29
    Vector { handler: DefaultExceptionHandler }, // 30
    Vector { handler: DefaultExceptionHandler }, // 31
    Vector { handler: DefaultExceptionHandler }, // 32
    Vector { handler: DefaultExceptionHandler }, // 33
    Vector { handler: DefaultExceptionHandler }, // 34
    Vector { handler: DefaultExceptionHandler }, // 35
    Vector { handler: DefaultExceptionHandler }, // 36
    Vector { handler: DefaultExceptionHandler }, // 37
    Vector { handler: DefaultExceptionHandler }, // 38
    Vector { handler: DefaultExceptionHandler }, // 39
    Vector { handler: DefaultExceptionHandler }, // 40
    Vector { handler: DefaultExceptionHandler }, // 41
    Vector { handler: DefaultExceptionHandler }, // 42
    Vector { handler: DefaultExceptionHandler }, // 43
    Vector { handler: DefaultExceptionHandler }, // 44
    Vector { handler: DefaultExceptionHandler }, // 45
    Vector { handler: DefaultExceptionHandler }, // 46
    Vector { handler: DefaultExceptionHandler }, // 47
    Vector { handler: DefaultExceptionHandler }, // 48
    Vector { handler: DefaultExceptionHandler }, // 49
    Vector { handler: DefaultExceptionHandler }, // 50
    Vector { handler: DefaultExceptionHandler }, // 51
    Vector { handler: DefaultExceptionHandler }, // 52
    Vector { handler: DefaultExceptionHandler }, // 53
    Vector { handler: DefaultExceptionHandler }, // 54
    Vector { handler: DefaultExceptionHandler }, // 55
    Vector { handler: DefaultExceptionHandler }, // 56
    Vector { handler: DefaultExceptionHandler }, // 57
    Vector { handler: DefaultExceptionHandler }, // 58
    Vector { handler: DefaultExceptionHandler }, // 59
    Vector { handler: DefaultExceptionHandler }, // 60
];


#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = HandlerReset;


// Reset Handler  first (and only) thing called when rebooting or starting up
#[no_mangle]
pub unsafe extern "C" fn HandlerReset() -> ! {    
	// configure clock
	// HSI clock = 16 MHz
	// HSE_VALUE <- 8 000 000 (8MHz - crystal frequency)
	// PLL_M <- 8 (/8 - 1MHz PLL)
	// MCK = (HS[EI]_CK / PLL_M) * PLL_N / PLL_P
	//	MCK		APB1	APB2	PLL_M	PLL_N	PLL_P	PLL_Q
	//	168		42		84		8		336		2		7
    
	// configure HSI clock
    rcc_cr_seti(RCC_CR_HSION);
    while (rcc_cr_read() & RCC_CR_HSIRDY) == 0 {}
    rcc_cfgr_set(0,0b00);
    
	// configure HSE clock
    rcc_cr_seti(RCC_CR_HSEON);
    while (rcc_cr_read() & RCC_CR_HSERDY) == 0 {}
    
	// configure AHB and AHP[12]
    rcc_cfgr_set(4,0b0000);
    rcc_cfgr_set(10,0b110);
    rcc_cfgr_set(13,0b100);
    
	// configure PLL
    rcc_cr_seti(!RCC_CR_PLLON);
    let mut x : u32 = 0;
    x = rep_bits(x,0,5,8);
    x = rep_bits(x,6,9,336);
    x = rep_bits(x,16,2,0);
    x |= RCC_PLLCFGR_PLLSRC;
    x = rep_bits(x,24,4,7);
    rcc_pllcfgr_write(x);
    rcc_cr_seti(RCC_CR_PLLON);
    while (rcc_cr_read() & RCC_CR_PLLRDY) == 0 {}
    
	// configure flash
	x = flash_acr_read();
	x |= FLASH_ACR_DCEN;
	x |= FLASH_ACR_ICEN;
    x = rep_bits(x,FLASH_ACR_LATENCY,2,5);
    flash_acr_write(x);
	
	// select PLL as SYSCLK
    rcc_cfgr_set(0,0b10);
    while get_bits(rcc_cfgr_read(),2,2) != 0b10 {}   
    rcc_cr_seti(!RCC_CR_HSION);

	// rmap SRAM at 0
    syscfg_memrmp_write(0b11);

	// copy data from FLASH to RAM
    let count1 = &_data_end as *const u8 as usize - &_data_begin as *const u8 as usize;
    ptr::copy_nonoverlapping(&_data_flash as *const u8, &mut _data_begin as *mut u8, count1);
    
	// set to 0 BSS
    let count2 = &_bss_end as *const u8 as usize - &_bss_begin as *const u8 as usize;
    ptr::write_bytes(&mut _bss_begin as *mut u8, 0, count2);


	// console configuration
    dcb_demcr_seti(DCB_DEMCR_TRCENA);
    itm_ter_seti(ITM_TRACE_EN_PORT0);

	extern "Rust" {
        fn main() -> !;
    }
    main()
}


// default handler if something goes wrong
#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
