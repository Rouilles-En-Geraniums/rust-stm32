#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

extern crate geranium_rt;
extern crate core;
use core::arch::asm;

pub mod stm32rustlib;

use crate::stm32rustlib::adc::*;
use crate::stm32rustlib::dac::*;
use crate::stm32rustlib::exti::*;
use crate::stm32rustlib::gpio::*;
use crate::stm32rustlib::nvic::*;
use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::tim::*;
use crate::stm32rustlib::various::*;

const APB1_CLK: u32 = 42_000_000;

const PSC: u32 = 1000;
const PERIOD: u32 = APB1_CLK / 1000; // TODO : define APB1_CLK
const HALF_PERIOD: u32 = PERIOD / 2;

const my_led: (char,u32) = ('D', 12);

#[no_mangle]
pub unsafe extern "C" fn handle_TIM4() {
	if (tim4_sr_read() & TIM_UIF) != 0 {
		if digital_read(my_led) == LOW {
			digital_write(my_led, HIGH);
		} else {
            digital_write(my_led, LOW);
		}
	}
	tim4_sr_write(tim4_sr_read() & !TIM_UIF);
	nvic_icpr_set(TIM4 >> 5, TIM4);
}


#[no_mangle]
fn init_TIM4_interrupt(){

    tim4_cr1_write(0);

	//DISABLE_IRQS; 
	unsafe {
		asm!("CPSID I");
	}
	
    // Configure NVIC 
	nvic_icer_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
	nvic_handler_set(TIM4, handle_TIM4);
	nvic_ipr_set(TIM4,0);
	
    // Purge pending IRQ
	nvic_icpr_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
    
	
	// configure TIM4
	tim4_cr1_write(tim4_cr1_read() & !TIM_CEN);
    tim4_psc_write(PSC - 1);
    tim4_arr_write(HALF_PERIOD);
    tim4_egr_write(TIM_UG);
    tim4_sr_write(0);
	
    // Enable IRQ
	nvic_iser_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
	
	tim4_dier_write(TIM_UIE);
	
	unsafe {
		asm!("CPSIE I");
	}

    tim4_cr1_write(tim4_cr1_read() | TIM_CEN)
}



#[no_mangle]
fn main() {
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 0)); //GPIO A
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 3)); //GPIO D
	rcc_apb1enr_write(rcc_ahb1enr_read() | (1 << 2)); //tim4en
    
    gpiod_moder_write(rep_bits(gpiod_moder_read(), my_led.1*2, 2, GPIO_MODER_OUT));

	init_TIM4_interrupt();

    loop {
    }
}