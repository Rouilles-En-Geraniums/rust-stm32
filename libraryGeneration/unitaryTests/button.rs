#![no_std]
#![no_main]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::system::*;



const PRESSED: u8 = 1;
const RELEASED: u8 = 0;


#[no_mangle]
fn main() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    
    let my_led = ('D', 12); // Built-in green led
    let my_but = ('A', 0); // Built-in blue button

    gpiod_moder_set(my_led.1*2, GPIO_MODER_OUT);

    gpioa_moder_set(my_but.1*2, GPIO_MODER_IN);
    gpioa_pupdr_set(my_but.1*2, GPIO_PUPDR_PD);

    let mut bstate = RELEASED;
    digital_write(my_led, LOW);
    loop {
        if bstate == RELEASED {
            if digital_read(my_but) == HIGH {
                bstate = PRESSED;
                digital_write(my_led, HIGH);
            }
        } else {
            if digital_read(my_but) == LOW {
                bstate = RELEASED;
                digital_write(my_led, LOW);
            }
        }
    }
}
