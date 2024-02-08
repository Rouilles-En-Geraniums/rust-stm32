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

#[no_mangle]
fn mywait(t: u32) { // milliseconds

    let psc: u32 = APB1_CLK / 1000;
    let period: u32 = t;

    rcc_apb1enr_write(rcc_apb1enr_read() | (1 << 2)); //tim4en

    tim4_cr1_write(tim4_cr1_read() & !TIM_CEN);
    tim4_psc_write(psc - 1);
    tim4_arr_write(period);
    tim4_egr_write(TIM_UG);
    tim4_sr_write(0);
    tim4_cr1_write(tim4_cr1_read() | TIM_CEN);
    
    while (tim4_sr_read() & TIM_UIF) == 0 {};

    tim4_sr_write(0);
    tim4_cr1_write(tim4_cr1_read() & !TIM_CEN);
}




#[no_mangle]
fn main() {
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 3)); //GPIO D
    
    let my_led = ('D', 12); // Built-in green led
    gpiod_moder_write(rep_bits(gpiod_moder_read(), my_led.1*2, 2, GPIO_MODER_OUT));
    
    digital_write(my_led, LOW);
    loop {
        digital_write(my_led, LOW);
        mywait(1000);
        digital_write(my_led, HIGH);
        mywait(1000);
    }
}