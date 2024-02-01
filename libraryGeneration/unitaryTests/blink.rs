#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m::asm;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::various;

pub mod stm32rustlib;

#[entry]
fn main() -> ! {
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); //GPIO D
    
    let my_led = ('D', 12); // Built-in green led

    gpio::gpiod_moder_write(various::rep_bits(gpio::gpiod_moder_read(), my_led.1*2, 2, gpio::GPIO_MODER_OUT));

    let mut bstate = RELEASED;
    gpio::digital_write(my_led, various::LOW);
    loop {
		gpio::digital_write(my_led, various::LOW);
		asm::delay(4200000);
		gpio::digital_write(my_led, various::HIGH);
		asm::delay(4200000);
    }
}
