use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::tim::*;
use crate::stm32rustlib::system::*;


const PSC_MS: u32 = APB1_CLK / 1_000;
const PSC_US: u32 = APB1_CLK / 1_000_000;


/**
 * This function must be called once before using any of the wait functions.
 */
pub fn wait_init_timers(){
    rcc_apb1enr_write(rcc_apb1enr_read() | RCC_APB1ENR_TIM2EN);
}

pub fn wait_ms(ms: u32) {
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(PSC_MS - 1);
    tim2_arr_write(ms);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
    
    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
}

pub fn wait_us(us: u32) {
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(PSC_US - 1);
    tim2_arr_write(us);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
    
    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
}

pub fn timer_arm_ms(ms: u32) {
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(PSC_MS - 1);
    tim2_arr_write(ms);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
}

pub fn timer_arm_us(us: u32) {
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(PSC_US - 1);
    tim2_arr_write(us);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
}

pub fn timer_timeout() {
    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
}

pub fn timer_is_timeout() -> bool {
    if (tim2_sr_read() & TIM_UIF) == 0 {
        false
    } else {
        true
    }
}