
#![no_std]
#![no_main]

extern crate core ; 
extern crate geranium_rt; 

use geranium_rt::stm32rustlib::exti::*;
use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::nvic::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::stm32rustlib::system::*;
use geranium_rt::stm32rustlib::geranium_print::*;

//// GPIOD
//#define MR_F 0
//#define MR_B 1
//#define ML_F 2
//#define ML_B 3
//
const MR_F: (char,u32) = ('D', 0);
const MR_B: (char,u32) = ('D', 1);
const ML_F: (char,u32) = ('C', 2);
const ML_B: (char,u32) = ('D', 3);
//// GPIOD
//#define PWM_PIN_LEFT  6 //sur tim3_ch1
//#define PWM_PIN_RIGHT 7 //sur tim3_ch2
const PWM_PIN_LEFT: (char,u32) = ('A', 6);
const PWM_PIN_RIGHT: (char,u32) = ('A', 7);
//
//#define PWM_PSC 13 //14 - 1
//#define PWM_ARR 60000
const PWM_PSC: u32 = 13;
const PWM_ARR: u32 = 60000;
//
//#define MAX_SPEED 256 //puissance de 2 de préférence
const MAX_SPEED:u32 = 256;
//

//inline void reset_speed_mr(){
//	TIM3_CCR2 = 0;
//}
fn reset_speed_mr(){
    tim3_ccr2_seti(0);
}
//
//inline void reset_speed_ml(){
//    TIM3_CCR1 = 0;
//}
fn reset_speed_ml(){
    tim3_ccr1_seti(0);
}
//
//inline void reset_speed(){
//	reset_speed_ml();
//	reset_speed_mr();
//}
fn reset_speed(){
    reset_speed_ml();
    reset_speed_mr();
}
//
//void set_speed_pwm_mr(int x){
//	TIM3_CCR2 = PWM_ARR * x / MAX_SPEED;
//}
//
fn set_speed_pwm_mr(x: u32){
    tim3_ccr2_seti(PWM_ARR * x / MAX_SPEED);
}


////x de 0 à MAX_SPEED-1
//void set_speed_mr_positive(int x){
//	set_speed_pwm_mr(x);
//    GPIOD_BSRR = (1 << MR_F) + (1 << (MR_B + 16));
//}
fn set_speed_mr_positive(x: u32){
    gpiod_bsrr_write((1 << MR_F.1) + (1 << (MR_B.1 + 16)))
}
//
//void set_speed_mr_negative(int x){
//	set_speed_pwm_mr(x);
//    GPIOD_BSRR = (1 << MR_B) + (1 << (MR_F + 16));
//}
fn set_speed_mr_negative(x: u32){
    gpiod_bsrr_write((1 << MR_B.1) + (1 << (MR_F.1 + 16)))
}
//
//void set_speed_pwm_ml(int x){
//	TIM3_CCR1 = PWM_ARR * x / MAX_SPEED;
//}
fn set_speed_pwm_ml(x: u32){
    tim3_ccr1_seti(PWM_ARR * x / MAX_SPEED);
}
//
//void set_speed_ml_positive(int x){
//	set_speed_pwm_ml(x);
//    GPIOD_BSRR = (1 << ML_F) + (1 << (ML_B + 16));
//}
fn set_speed_ml_positive(x: u32){
    gpiod_bsrr_write((1 << ML_F.1) + (1 << (ML_B.1 + 16)))
}
//
//void set_speed_ml_negative(int x){
//	set_speed_pwm_ml(x);
//    GPIOD_BSRR = (1 << ML_B) + (1 << (ML_F + 16));
//}
fn set_speed_ml_negative(x: u32){
    gpiod_bsrr_write((1 << ML_B.1) + (1 << (ML_F.1 + 16)))
}
//
////x de -MAX_SPEED+1 à MAX_SPEED-1
//void set_speed_mr(int x){
//	if (x < 0){
//		set_speed_mr_negative(-x);
//	} else {
//		set_speed_mr_positive(x);
//	}
//}
fn set_speed_mr(x: u32){
    if x < 0  {
        set_speed_mr_negative((0 as u32)-x);
    }
    else{
        set_speed_mr_positive(x);
    }

}
//
//void set_speed_ml(int x){
//	if (x < 0){
//		set_speed_ml_negative(-x);
//	} else {
//		set_speed_ml_positive(x);
//	}
//}
fn set_speed_ml(x: u32){
    if x < 0  {
        set_speed_ml_negative((0 as u32)-x);
    }
    else{
        set_speed_ml_positive(x);
    }

}
//
//void turn_left(int x){
//	set_speed_ml_negative(x);
//	set_speed_mr_positive(x);
//}
fn turn_left(x: u32){
    set_speed_ml_negative(x);
    set_speed_mr_positive(x);
}
//
//void turn_right(int x){
//	set_speed_mr_negative(x);
//	set_speed_ml_positive(x);
//}
fn turn_right(x: u32){
	set_speed_mr_negative(x);
	set_speed_ml_positive(x);
}
//
//void init_enable_pins(){
//	GPIOD_MODER = REP_BITS(GPIOD_MODER, 2 * ML_F, 2, GPIO_MODER_OUT);
//	GPIOD_OTYPER = GPIOD_OTYPER & ~ (1 << ML_F);
//	GPIOD_MODER = REP_BITS(GPIOD_MODER, 2 * ML_B, 2, GPIO_MODER_OUT);
//	GPIOD_OTYPER = GPIOD_OTYPER & ~ (1 << ML_B);
//	GPIOD_MODER = REP_BITS(GPIOD_MODER, 2 * MR_F, 2, GPIO_MODER_OUT);
//	GPIOD_OTYPER = GPIOD_OTYPER & ~ (1 << MR_F);
//	GPIOD_MODER = REP_BITS(GPIOD_MODER, 2 * MR_B, 2, GPIO_MODER_OUT);
//	GPIOD_OTYPER = GPIOD_OTYPER & ~ (1 << MR_B);
//}
//
fn init_enable_pins(){
    gpiod_moder_set(ML_F.1*2,2,GPIO_MODER_OUT);
    gpiod_moder_set(ML_B.1*2,2,GPIO_MODER_OUT);
    gpiod_moder_set(MR_F.1*2,2,GPIO_MODER_OUT);
    gpiod_moder_set(MR_B.1*2,2,GPIO_MODER_OUT);

}
//void init_PWM(){
//	TIM3_CR1 = 0;
//	TIM3_ARR = PWM_ARR;
//	TIM3_PSC = PWM_PSC;
//	TIM3_EGR = TIM_UG;
//	TIM3_CCMR1 = TIM_CCS1S_OUT|TIM_OC1M_PWM1 | TIM_CCS2S_OUT|TIM_OC2M_PWM1;
//	TIM3_CCER = TIM_CC1E|TIM_CC1NP | TIM_CC2E|TIM_CC2NP;
//	reset_speed();
//	TIM3_SR = 0;
//	TIM3_CR1 = TIM_CEN | TIM_ARPE;
//
//    GPIOA_MODER = REP_BITS(GPIOA_MODER, 2 * PWM_PIN_LEFT, 2, GPIO_MODER_ALT);
//    GPIOA_AFRL = REP_BITS(GPIOA_AFRL, 4 * PWM_PIN_LEFT, 4, 2);
//    GPIOA_MODER = REP_BITS(GPIOA_MODER, 2 * PWM_PIN_RIGHT, 2, GPIO_MODER_ALT);
//    GPIOA_AFRL = REP_BITS(GPIOA_AFRL, 4 * PWM_PIN_RIGHT, 4, 2);
//}
fn init_PWM(){
    tim3_cr1_write(0); 
    tim3_psc_write(PWM_ARR);
    tim3_arr_write(PWM_PSC);
    tim3_egr_write(TIM_UG);
    tim3_ccmr1_seti(TIM_CCS1S_OUT|TIM_OC1M_PWM1 | TIM_CCS2S_OUT|TIM_OC2M_PWM1);
    tim3_ccer_seti(TIM_CC1E|TIM_CC1NP | TIM_CC2E|TIM_CC2NP);
    reset_speed();
    tim3_sr_seti(0);
    tim3_cr1_seti(TIM_CEN | TIM_ARPE);

    gpioa_moder_set(PWM_PIN_LEFT.1*2,2,GPIO_MODER_ALT);
    gpioa_afrh_set(PWM_PIN_LEFT.1*4,4,2);
    gpioa_moder_set(PWM_PIN_RIGHT.1*2,2,GPIO_MODER_ALT);
    gpioa_afrh_set(PWM_PIN_RIGHT.1*4,4,2);
}

//int main() {
//	printf("\nStarting...\n");
//
//	// RCC init
//	RCC_AHB1ENR |= RCC_GPIOAEN;
//	RCC_AHB1ENR |= RCC_GPIODEN;
//	RCC_APB1ENR |= RCC_TIM3EN;
//	RCC_APB1ENR |= RCC_TIM5EN;
//	RCC_APB2ENR |= RCC_ADC1EN;
//
//	// initialization
//
// 	init_PWM();
//	init_enable_pins();
//	turn_left(20);
//
//	// main loop
//	printf("Endless loop!\n");
//	while(1) {
//	}
//
//}

#[no_mangle]
fn main()  {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOCEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM3EN);
    
    loop {

    }

}