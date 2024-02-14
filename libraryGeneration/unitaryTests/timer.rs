#![no_std]
#![no_main]

extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::system::*;

const PSC: u32 = 1000;
const PERIOD: u32 = APB1_CLK / 1000;

pub fn init_timer() {
    tim4_cr1_seti(!TIM_CEN);
    tim4_psc_write(PSC - 1);
    tim4_arr_write(PERIOD);
    tim4_egr_write(TIM_UG);
    tim4_sr_write(0);
    tim4_cr1_seti(TIM_CEN);
}


#[no_mangle]
fn main() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM4EN);
    
    let my_led = ('D', 12); // Built-in green led
    gpiod_moder_set(my_led.1*2, GPIO_MODER_OUT);

    init_timer();

    digital_write(my_led, LOW);
    loop {
        while (tim4_sr_read() & TIM_UIF) == 0 {};

        if digital_read(my_led) == LOW {
			digital_write(my_led, HIGH);
		} else {
            digital_write(my_led, LOW);
		}
        tim4_sr_write(0);
    }
}