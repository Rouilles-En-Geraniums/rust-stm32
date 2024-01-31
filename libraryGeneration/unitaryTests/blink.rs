#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;

use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;

use crate::stm32rustlib::gpio;
pub mod stm32rustlib;


const RCC_AHB1ENR : u32 = 0x40023830 ;
pub fn rcc_write(value: u32) {
    unsafe {
        write_volatile( RCC_AHB1ENR as *mut u32, value )
    };
}
pub fn rcc_read() -> u32 {
    unsafe {
        read_volatile( RCC_AHB1ENR as *mut u32 )
    }
}


#[entry]
fn main() -> ! {
    rcc_write(rcc_read() | (1 << 3));
    
    let my_led = ('D', 2);

    gpio::gpiod_moder_write(1 << (my_led.1*2));
    // GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));

    let mut i;
    loop {
        gpio::gpiod_bsrr_write(1 << my_led.1);
        i = 0;
        while i < 3000000 {
            i+=1;
        }
        gpio::gpiod_bsrr_write(1 << (my_led.1 + 16));
        i = 0;
        while i < 3000000 {
            i+=1;
        }
    }
}
