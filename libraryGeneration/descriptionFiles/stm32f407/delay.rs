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

use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::tim::*;
use crate::stm32rustlib::system::*;
use crate::stm32rustlib::systick::*;
use crate::println;

const PSC_MS: u32 = APB1_CLK / 1_000;
const PSC_US: u32 = APB1_CLK / 1_000_000;

static mut TIME_MS: u32 = 0;

/**
 * This function must be called once before using any of the delay functions.
 */
#[inline(always)]
pub fn delay_init_timers(){
    rcc_apb1enr_seti(RCC_APB1ENR_TIM2EN);
    unsafe { TIME_MS = 0; } // safety measure
    
    println!("init timer");
    
    systick_rvr_write(MCK_CLK/1000-1);//RVR : Reload Value Register
    //systick_rvr_write(18_750);//RVR : Reload Value Register
    println!("rvr:{:#032b}",systick_rvr_read()); 

    systick_cvr_write(0x00);
    println!("cvr:{:#032b}",systick_cvr_read());

    systick_csr_seti(SYSTICK_CSR_TICKINT);
    systick_csr_seti(SYSTICK_CSR_CLKSOURCE);

    systick_csr_seti(SYSTICK_CSR_ENABLE);
    println!("csr:{:#032b}",systick_csr_read());
}

#[no_mangle]
pub fn systick_handler(){
    unsafe { TIME_MS += 1; }
}

pub fn millis() -> u32 {
    unsafe { TIME_MS }
}

#[inline(always)]
pub fn delay_ms(ms: u32) {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if ms == 0 { return ; }
    tim2_cr1_seti(!TIM_CEN);
    tim2_psc_write(PSC_MS - 1);
    tim2_arr_write(ms - 1);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_seti(TIM_CEN);

    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn delay_us(us: u32) {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if us == 0 { return ; }
    tim2_cr1_seti(!TIM_CEN);
    tim2_psc_write(PSC_US - 1);
    tim2_arr_write(us - 1);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_seti(TIM_CEN);

    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn timer_arm_ms(ms: u32) {
    tim2_cr1_seti(!TIM_CEN);
    tim2_psc_write(PSC_MS - 1);
    tim2_arr_write(ms);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_seti(TIM_CEN);
}

#[inline(always)]
pub fn timer_arm_us(us: u32) {
    tim2_cr1_seti(!TIM_CEN);
    tim2_psc_write(PSC_US - 1);
    tim2_arr_write(us);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_seti(TIM_CEN);
}

#[inline(always)]
pub fn timer_timeout() {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if tim2_arr_read() == 0 { return; }

    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn timer_is_timeout() -> bool {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if tim2_arr_read() == 0 {
        true
    } else {
        (tim2_sr_read() & TIM_UIF) != 0
    }
}
