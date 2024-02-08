#![no_std]
#![no_main]
extern crate core;

use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;
use panic_halt as _;

use crate::stm32rustlib::gpio;
use crate::stm32rustlib::rcc;
use crate::stm32rustlib::tim;
use crate::stm32rustlib::various;
use cortex_m::interrupt;

pub mod stm32rustlib;

const PRESSED: u8 = 1;
const RELEASED: u8 = 0;
const PSC: u32 = 1000;
const PERIOD: u32 = (42 * 1_000_000) / PSC;
const HALF_PERIOD: u32 = PERIOD / 2;
const MY_LED: (char, u32) = ('D', 12);
const MY_BUT: (char, u32) = ('A', 0);
const QUIT: u32 = PERIOD / 4;

fn init_tim4() {
    tim::tim4_cr1_write(0);
    tim::tim4_psc_write(PSC - 1);
    tim::tim4_arr_write(HALF_PERIOD);
    tim::tim4_egr_write(tim::TIM_UG);
    tim::tim4_sr_write(0);
    tim::tim4_cr1_write(tim::tim4_cr1_read() | tim::TIM_CEN)
}

#[entry]
fn main() -> ! {
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 0)); // GPIO A
    rcc::rcc_ahb1enr_write(rcc::rcc_ahb1enr_read() | (1 << 3)); // GPIO D
    rcc::rcc_apb1enr_write(rcc::rcc_apb1enr_read() | (1 << 2));

    gpio::gpiod_moder_write(various::rep_bits(gpio::gpiod_moder_read(), MY_LED.1 * 2, 2, gpio::GPIO_MODER_OUT));
    gpio::gpioa_moder_write(various::rep_bits(gpio::gpioa_moder_read(), MY_BUT.1 * 2, 2, gpio::GPIO_MODER_IN));
    gpio::gpioa_pupdr_write(various::rep_bits(gpio::gpioa_pupdr_read(), MY_BUT.1 * 2, 2, gpio::GPIO_PUPDR_PD));

    init_tim4();
    let mut half_period = HALF_PERIOD;
    let mut period = PERIOD;
    let mut quit = QUIT;
    let mut bstate = RELEASED;
    loop {
        while tim::tim4_sr_read() & tim::TIM_UIF == 0 {
            if gpio::digital_read(MY_BUT) != various::LOW {
                bstate = PRESSED;
            } else if bstate == PRESSED {
                hprintln!("PRESSED").unwrap();
                bstate = RELEASED;
                half_period = half_period / 2;
                if half_period < quit {
                    half_period = period;
                }
            }
        }
        if gpio::digital_read(MY_LED) == various::LOW {
            gpio::digital_write(MY_LED, various::HIGH);
        } else {
            gpio::digital_write(MY_LED, various::LOW);
        }
        tim::tim4_arr_write(half_period);
        hprintln!("Current arr Value :{}",tim::tim4_arr_read()).unwrap();
        tim::tim4_sr_write(0);
    }
}
