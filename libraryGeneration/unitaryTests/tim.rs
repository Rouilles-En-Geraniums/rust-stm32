#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::tim;

pub mod stm32rustlib;

// bit range macros
fn mask(l: u32) -> u32 {
    (1 << l) - 1
}

fn get_bits(x: u32, i: u32, l: u32) -> u32 {
    (x >> i) & mask(l)
}

fn rep_bits(x: u32, i: u32, l: u32, y: u32) -> u32 {
    (x & !(mask(l) << i)) | (y << i)
}

const GPIO_MODER_IN: u32 = 0b00;
const GPIO_MODER_OUT: u32 = 0b01;
const GPIO_PUPDR_NO: u32 = 0b00;
const GPIO_PUPDR_PU: u32 = 0b01;
const GPIO_PUPDR_PD: u32 = 0b10;

const PRESSED: u8 = 1;
const RELEASED: u8 = 0;
const WAIT_PSC: u32 = 1000;
const WAIT_DELAY: u32 = ((42 * 1000000) / WAIT_PSC) / 2;

fn init_tim4() {
    tim::tim4_cr1_write(0);
    tim::tim4_psc_write(WAIT_PSC - 1);
    tim::tim4_arr_write(1000000);
    tim::tim4_egr_write(tim::TIM_UG);
    tim::tim4_sr_write(0);
}

#[entry]
fn main() -> ! {
    // RCC init
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 0)); // GPIO A
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); // GPIO D
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 2));
    let my_led = ('D', 13);
    // GPIO init
    gpio::gpiod_moder_write(rep_bits(gpio::gpiod_moder_read(), my_led.1*2, 2, GPIO_MODER_OUT));
    //TIM  init 
    init_tim4();
    tim::tim4_cr1_write(tim::TIM_CEN);
    loop {
        if gpio::gpiod_odr_read() & (1<<my_led.1) == 0{
            gpio::gpiod_bsrr_write(1 << my_led.1);
        }
        else{
            gpio::gpiod_bsrr_write(1 << (my_led.1 + 16));
        }
        
    tim::tim4_sr_write(0);
        
    }
}    