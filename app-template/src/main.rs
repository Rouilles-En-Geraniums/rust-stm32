#![no_std]
#![no_main]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::wait::*;


#[no_mangle]
fn main() {
    wait_init_timers();
    
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