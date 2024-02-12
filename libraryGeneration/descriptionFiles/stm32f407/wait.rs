use crate::stm32rustlib::various::*;
use crate::stm32rustlib::rcc::*;
use crate::stm32rustlib::tim::*;


// STM32f4 Discovery
pub const MCK_CLK: u32 = 168_000_000;
pub const APB1_CLK: u32 = 42_000_000;
pub const APB2_CLK: u32 = 84_000_000;

// Tim2
pub fn wait_ms(t: u32) { // milliseconds

    let psc: u32 = APB1_CLK / 1000;
    let period: u32 = t;

    rcc_apb1enr_write(rcc_apb1enr_read() | (1 << 0)); //tim2en

    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(psc - 1);
    tim2_arr_write(period);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
    
    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
}

pub fn timer_arm(t: u32)  { // milliseconds
    let psc: u32 = APB1_CLK / 1000;
    let period: u32 = t;

    rcc_apb1enr_write(rcc_apb1enr_read() | (1 << 0)); //tim2en

    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
    tim2_psc_write(psc - 1);
    tim2_arr_write(period);
    tim2_egr_write(TIM_UG);
    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() | TIM_CEN);
}

pub fn timer_timeout(){
    while (tim2_sr_read() & TIM_UIF) == 0 {};

    tim2_sr_write(0);
    tim2_cr1_write(tim2_cr1_read() & !TIM_CEN);
}

pub fn timer_isTimeout() -> bool {
    if (tim2_sr_read() & TIM_UIF) == 0 {
        false
    } else {
        true
    }
}