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
use crate::stm32rustlib::gpio::*;
use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::various::*;
use crate::stm32rustlib::wait::*;


#[no_mangle]
fn main() {
    rcc_ahb1enr_write(rcc_ahb1enr_read() | RCC_AHB1ENR_GPIODEN);
    
    let my_led = ('D', 12); // Built-in green led
    gpiod_moder_write(rep_bits(gpiod_moder_read(), my_led.1*2, 2, GPIO_MODER_OUT));
    
    digital_write(my_led, LOW);
    loop {
        digital_write(my_led, LOW);
        wait_ms(1000);
        digital_write(my_led, HIGH);
        wait_ms(1000);
    }
}