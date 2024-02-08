#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::various;

pub mod stm32rustlib;

const PRESSED: u8 = 1;
const RELEASED: u8 = 0;

#[entry]
fn main() -> ! {
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 0)); //GPIO A
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); //GPIO D
    
    let my_led = ('D', 12); // Built-in green led
    let my_but = ('A', 0); // Built-in blue button

    gpio::gpiod_moder_write(various::rep_bits(gpio::gpiod_moder_read(), my_led.1*2, 2, gpio::GPIO_MODER_OUT));

    gpio::gpioa_moder_write(various::rep_bits(gpio::gpioa_moder_read(), my_but.1*2, 2, gpio::GPIO_MODER_IN));
    gpio::gpioa_pupdr_write(various::rep_bits(gpio::gpioa_pupdr_read(), my_but.1*2, 2, gpio::GPIO_PUPDR_PD));

    let mut bstate = RELEASED;
    gpio::digital_write(my_led, various::LOW);
    loop {
        if bstate == RELEASED {
            if gpio::digital_read(my_but) == various::HIGH {
                bstate = PRESSED;
                gpio::digital_write(my_led, various::HIGH);
            }
        } else {
            if gpio::digital_read(my_but) == various::LOW {
                bstate = RELEASED;
                gpio::digital_write(my_led, various::LOW);
            }
        }
    }
}
