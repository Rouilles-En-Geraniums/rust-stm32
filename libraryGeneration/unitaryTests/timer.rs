#![no_std]
#![no_main]
extern crate core;
use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::tim;
use crate::stm32rustlib::various;
use cortex_m::interrupt;

pub mod stm32rustlib;

const PSC: u32 = 1000;
const PERIOD: u32 = (42 * 1000000) / PSC;
const HALF_PERIOD: u32 = PERIOD / 2;

const my_led: (char,u32) = ('D', 12);

fn init_TIM4(){
    tim::tim4_cr1_write(0);

	tim::tim4_cr1_write(tim::tim4_cr1_read() & !tim::TIM_CEN);
    tim::tim4_psc_write(PSC - 1);
    tim::tim4_arr_write(HALF_PERIOD);
    tim::tim4_egr_write(tim::TIM_UG);
    tim::tim4_sr_write(0);

    tim::tim4_cr1_write(tim::tim4_cr1_read() | tim::TIM_CEN)
}


#[entry]
fn main() -> ! {
	// RCC init
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 0)); //GPIO A
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); //GPIO D
	rcc::rcc_apb1enr_write(rcc::rcc_apb1enr_read() | (1 << 2)); //tim4en
    
	gpio::gpiod_moder_write(various::rep_bits(gpio::gpiod_moder_read(), my_led.1*2, 2, gpio::GPIO_MODER_OUT));
	
	init_TIM4();
	
	loop {
		while (tim::tim4_sr_read() & tim::TIM_UIF == 0) {}
		if gpio::digital_read(my_led) == various::LOW {
			gpio::digital_write(my_led, various::HIGH);
		} else {
            gpio::digital_write(my_led, various::LOW);
		}
		tim::tim4_sr_write(0);
	}
}
