#![no_std]
#![no_main]

extern crate geranium_rt;
extern crate core;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::system::*;
use geranium_rt::stm32rustlib::nvic::*;
use geranium_rt::stm32rustlib::tim::*;

use core::arch::asm;


const PSC: u32 = 1000;
const PERIOD: u32 = APB1_CLK / 1000; // TODO : define APB1_CLK
const HALF_PERIOD: u32 = PERIOD / 2;

const MY_LED: (char,u32) = ('D', 12);

#[no_mangle]
pub unsafe extern "C" fn handle_TIM4() {
	if (tim4_sr_read() & TIM_UIF) != 0 {
		if digital_read(MY_LED) == LOW {
			digital_write(MY_LED, HIGH);
		} else {
            digital_write(MY_LED, LOW);
		}
	}
	tim4_sr_seti(!TIM_UIF);
	nvic_icpr_set(TIM4 >> 5, TIM4);
}


#[no_mangle]
fn init_tim4_interrupt(){

    tim4_cr1_write(0);

	//DISABLE_IRQS; 
	unsafe {
		asm!("CPSID I");
	}
	
    // Configure NVIC 
	nvic_icer_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
	nvic_handler_set(TIM4, handle_TIM4);
	nvic_ipr_set(TIM4,0);
	
    // Purge pending IRQ
	nvic_icpr_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
    
	
	// configure TIM4
	tim4_cr1_write(tim4_cr1_read() & !TIM_CEN);
    tim4_psc_write(PSC - 1);
    tim4_arr_write(HALF_PERIOD);
    tim4_egr_write(TIM_UG);
    tim4_sr_write(0);
	
    // Enable IRQ
	nvic_iser_set(TIM4 >> 5, 1 << (TIM4 & 0x1F));
	
	tim4_dier_write(TIM_UIE);
	
	unsafe {
		asm!("CPSIE I");
	}

    tim4_cr1_seti(TIM_CEN)
}



#[no_mangle]
fn main() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM4EN);
    
    gpiod_moder_set(MY_LED.1*2, GPIO_MODER_OUT);

	init_tim4_interrupt();

    loop {
    }
}