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

use core::arch::asm;
use core::panic;

use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::tim::*;
use crate::stm32rustlib::system::*;
use crate::stm32rustlib::nvic::*;


const PSC_MS: u32 = APB1_CLK / 1_000;
const PSC_US: u32 = APB1_CLK / 1_000_000;


/**
 * This function must be called once before using any of the delay functions.
 */
#[inline(always)]
pub fn seq_delay_init_timers(){
    rcc_apb1enr_seti(RCC_APB1ENR_TIM5EN);
}

#[inline(always)]
pub fn seq_delay_ms(ms: u32) {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if ms == 0 { return ; }
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_MS - 1);
    tim5_arr_write(ms);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);
    tim5_cr1_seti(TIM_CEN);

    while (tim5_sr_read() & TIM_UIF) == 0 {};

    tim5_sr_write(0);
    tim5_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn seq_delay_us(us: u32) {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if us == 0 { return ; }
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_US - 1);
    tim5_arr_write(us);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);
    tim5_cr1_seti(TIM_CEN);

    while (tim5_sr_read() & TIM_UIF) == 0 {};

    tim5_sr_write(0);
    tim5_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn seq_timer_arm_ms(ms: u32) {
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_MS - 1);
    tim5_arr_write(ms);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);
    tim5_cr1_seti(TIM_CEN);
}

#[inline(always)]
pub fn seq_timer_arm_us(us: u32) {
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_US - 1);
    tim5_arr_write(us);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);
    tim5_cr1_seti(TIM_CEN);
}

#[inline(always)]
pub fn seq_timer_timeout() {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if tim5_arr_read() == 0 { return; }

    while (tim5_sr_read() & TIM_UIF) == 0 {};

    tim5_sr_write(0);
    tim5_cr1_seti(!TIM_CEN);
}

#[inline(always)]
pub fn seq_timer_is_timeout() -> bool {
    // ST Ref. Man. RM0090 section 18.4.12 :
    // "The CNT is blocked while ARR is null"
    if tim5_arr_read() == 0 {
        true
    } else {
        (tim5_sr_read() & TIM_UIF) != 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_TIM5() {
    if (tim5_sr_read() & TIM_UIF) != 0 {
        tim5_sr_seti(!TIM_UIF);
        nvic_icpr_set(TIM4 >> 5, TIM4);
        panic!("Current task exceded its duration (WCET)");
    }
}

/**
 * This function must be called before using seq_timer_arm_[ms,us]_interrupt functions.
 * You still need to call seq_delay_init_timers()
 *
 * This function set the handler for the interrupt.
 * f must be declared as a pub unsafe extern "C" function
 * with #[no_mangle] attribute for it to be properly set as
 * a handler in the Interruption Vector Table.
 */
pub fn seq_timer_arm_interrupt_init() {
    // disable timer5
    tim5_cr1_seti(!TIM_CEN);

    // disable global interrupt
    unsafe { asm!("CPSID I"); }
    // disable interrupt for TIM5 NVIC
        // first 27 bits give the NVIC_ICER register adress
        // last 5 bits gives the bit in this register (2^5=32)
    nvic_icer_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));
    // disable interrupt in device TIM5
    tim5_dier_seti(!TIM_UIE);
    // clear pending interrupt for TIM5 NVIC
    nvic_icpr_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));
    // clear pending interrupt in device TIM5
    tim5_sr_write(0);
    // set handler for TIM5 NVIC
    nvic_handler_set(TIM5, handle_TIM5);
    // set priority for TIM5 NVIC
    nvic_ipr_set(TIM5, 0);
    // enable interrupt in device TIM5
    tim5_dier_write(TIM_UIE); // and clear other bts
    // enable interrupt for TIM5 NVIC
    nvic_iser_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));
    // enable interrupt globaly
    unsafe { asm!("CPSIE I"); }
}

// Disable interrupt for seq_timer_arm_[ms,us]_interrupt functions
// Doesn't reset timer, you can still use timer_timout functions
pub fn seq_disable_arm_interrupt() {
    // disable global interrupt
    unsafe { asm!("CPSID I"); }
    // disable interrupt for TIM5 NVIC
    nvic_icer_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));
    // disable interrupt in device TIM5
    tim5_dier_seti(!TIM_UIE);
    // clear pending interrupt for TIM5 NVIC
    nvic_icpr_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));
    // clear pending interrupt in device TIM5
    tim5_sr_write(0);
    // enable interrupt globaly
    unsafe { asm!("CPSIE I"); }
}

// Interrupt triggered by hardware when timer runs out and will panic!()
// Use seq_disable_arm_interrupt to disarm it
#[inline(always)]
pub fn seq_timer_arm_ms_interrupt(ms: u32) {
    // disable global interrupt
    unsafe { asm!("CPSID I"); }
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_MS - 1);
    tim5_arr_write(ms);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);

    // enable interrupt in device TIM5
    tim5_dier_write(TIM_UIE);
    // enable interrupt for TIM5 NVIC
    nvic_iser_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));

    tim5_cr1_seti(TIM_CEN);
    // enable interrupt globaly
    unsafe { asm!("CPSIE I"); }
}

// Interrupt triggered by hardware when timer runs out and will panic!()
// Use seq_disable_arm_interrupt to disarm it
#[inline(always)]
pub fn seq_timer_arm_us_interrupt(us: u32) {
    // disable global interrupt
    unsafe { asm!("CPSID I"); }
    tim5_cr1_seti(!TIM_CEN);
    tim5_psc_write(PSC_US - 1);
    tim5_arr_write(us);
    tim5_egr_write(TIM_UG);
    tim5_sr_write(0);

    // enable interrupt in device TIM5
    tim5_dier_write(TIM_UIE);
    // enable interrupt for TIM5 NVIC
    nvic_iser_set(TIM5 >> 5, 1 << (TIM5 & 0x1F));

    tim5_cr1_seti(TIM_CEN);
    // enable interrupt globaly
    unsafe { asm!("CPSIE I"); }
}
