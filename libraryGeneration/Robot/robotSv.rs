#![no_std]
#![no_main]

extern crate core ; 
extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::stm32rustlib::various::*;



const T3_PSC: u32 = 14;
//const PERIOD: u32 = APB1_CLK / 1000; // TODO : define APB1_CLK
const P3_20MS: u32 = (42 * 1000000) / 14; // TODO : define APB1_CLK
const PSERVO : u32 = P3_20MS;

const PIN_RIGHT_FORWARD: (char,u32) = ('D', 1);
const PIN_RIGHT_BACKWARD: (char,u32) = ('D', 0);
const PIN_RIGHT_MOTOR: (char,u32) = ('C', 6);
const PIN_LEFT_FORWARD: (char,u32) = ('D', 3);
const PIN_LEFT_BACKWARD: (char,u32) = ('D', 2);
const PIN_LEFT_MOTOR: (char,u32) = ('C', 7);
const AF2: u32 = 2;

fn init_timer(){
    	// Timer3 for servomotor PWMuse geranium_rt::stm32rustlib::geranium_print::*;

	// 20 ms Period, duty cycle [1,2]ms
    //TIM3_CR1 = 0;
    //TIM3_PSC = T3_PSC - 1;
    //TIM3_ARR = PSERVO;

    tim3_cr1_write(0); 
    tim3_psc_write(T3_PSC-1);
    tim3_arr_write(PSERVO);
    // Right Motor
	//TIM3_CCMR1 |= TIM_OC1M_PWM1; // channel 1 Mode P
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC1M_PWM1); 
    //tim3_ccmr1_seti(TIM_OC1M_PWM1);
	//TIM3_CCMR1 |= TIM_CCS1S_OUT; // channel direction output
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_CCS1S_OUT); 
    //tim3_ccmr1_seti(TIM_CCS1S_OUT);
	// we change the value of CCR1 so it's cleaner to have it prealoaded
	//TIM3_CCMR1 |= TIM_OC1PE; // buffered CCR1
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC1PE); 
    //tim3_ccmr1_seti(TIM_OC1PE);

	//TIM3_CCER |= TIM_CC1E; // OC1 signal is output on the corresponding output pin
    //tim3_ccer_write(tim3_ccer_read() | TIM_CC1E); 
    tim3_ccer_seti(TIM_CC1E);
	//TIM3_CCER &= ~(TIM_CC1P); // OC1 active high
    tim3_ccer_write(tim3_ccer_read() & !TIM_CC1P); 
    //tim3_ccer_seti(!TIM_CC1P);
	// set duty cycle to 0ms 
	//TIM3_CCR1 = 0; // = set_speed_right(0)
    tim3_ccr1_write(0); 
    //tim3_ccr1_seti(0);
    

    // Left Motor
	//TIM3_CCMR1 |= TIM_OC2M_PWM1; // channel 2 Mode PWM1
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC2M_PWM1);
    //tim3_ccmr1_seti(TIM_OC2M_PWM1);

	//TIM3_CCMR1 |= TIM_CCS2S_OUT; // channel direction output
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_CCS2S_OUT);
    //tim3_ccmr1_seti(TIM_CCS2S_OUT);

	// we change the value of CCR2 so it's cleaner to have it prealoaded
	//TIM3_CCMR1 |= TIM_OC2PE; // buffered CCR2
    //tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC2PE);
    tim3_ccmr1_seti(TIM_OC2PE);


	//TIM3_CCER |= TIM_CC2E; // OC2 signal is output on the corresponding output pin
    //tim3_ccer_write(tim3_ccer_read() | TIM_CC2E); 
    tim3_ccer_seti(TIM_CC2E);

	//TIM3_CCER &= ~(TIM_CC2P); // OC2 active high
    //tim3_ccer_write(tim3_ccer_read() & !TIM_CC2P); 
    tim3_ccer_seti( !TIM_CC2P);
    // set duty cycle to 0ms 
	//TIM3_CCR2 = PSERVO / 20; // = set_speed_left(0)
    tim3_ccr2_write(0); 
    //tim3_ccr2_seti(0);


	// Generate an event to store the value from preload register
	// to the shadow(real) register
	//TIM3_EGR = TIM_UG;
    tim3_egr_write(TIM_UG);
    //TIM3_SR = 0;
    tim3_sr_write(0); 
    //tim3_sr_seti(0);
	// turn on timer
	//TIM3_CR1 = TIM_CEN;
    tim3_cr1_write(TIM_CEN);
    //tim3_cr1_seti(TIM_CEN);
}

// speed must be in [0, 100]
fn set_speed_right(speed: u32) {

    tim3_ccr1_write(speed * PSERVO / 100 );
    //tim3_ccr1_seti(speed * PSERVO / 100);
	
}
// speed must be in [0, 100]
fn set_speed_left(speed: u32) {
    tim3_ccr2_write(speed * PSERVO / 100 );
    //tim3_ccr2_seti(speed * PSERVO / 100);

}



#[no_mangle]
fn main() {
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 0)); // GPIO A
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 3)); // GPIO D
    rcc_ahb1enr_write(rcc_ahb1enr_read() | (1 << 2)); // GPIO C
    rcc_apb1enr_write(rcc_apb1enr_read() | (1 << 1)); // TIM3
    //rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    //rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    //rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOCEN);
    //rcc_apb1enr_seti(RCC_APB1ENR_TIM3EN);


    /****************
     * RIGHT MOTOR *
    *****************/
    // configure directional control pins for right motor 
    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_RIGHT_FORWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_RIGHT_FORWARD.1*2,2,GPIO_MODER_OUT);
    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_RIGHT_BACKWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_RIGHT_BACKWARD.1*2,2,GPIO_MODER_OUT);

    // set alternative mode
    //gpioc_moder_write(rep_bits(gpioc_moder_read(), PIN_RIGHT_MOTOR.1 * 2, 2, GPIO_MODER_ALT));
    gpioc_moder_set(PIN_RIGHT_MOTOR.1*2,2,GPIO_MODER_ALT);

    if PIN_RIGHT_MOTOR.1 <= 7 {
        //gpioc_afrl_write(rep_bits(gpioc_afrl_read(),PIN_RIGHT_MOTOR.1 * 4,4,AF2));
        gpioc_afrl_set(PIN_RIGHT_MOTOR.1 * 4,4,AF2);
    }
    else{
        //gpioc_afrh_write(rep_bits(gpioc_afrh_read(),PIN_RIGHT_MOTOR.1 * 4,4,AF2));
        gpioc_afrh_set(PIN_RIGHT_MOTOR.1 * 4,4,AF2);

    }

    /***************
     * LEFT MOTOR *
    ****************/


    // configure directional control pins for left motor
    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_LEFT_FORWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_LEFT_FORWARD.1*2,2,GPIO_MODER_OUT);

    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_LEFT_BACKWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_LEFT_BACKWARD.1*2,2,GPIO_MODER_OUT);

    // set alternative mode
   // gpioc_moder_write(rep_bits(gpioc_moder_read(), PIN_LEFT_MOTOR.1 * 2, 2, GPIO_MODER_ALT));
    gpioc_moder_set(PIN_LEFT_MOTOR.1*2,2,GPIO_MODER_ALT);

    if PIN_LEFT_MOTOR.1 <= 7 {
        //gpioc_afrl_write(rep_bits(gpioc_afrl_read(),PIN_LEFT_MOTOR.1 * 4,4,AF2));
        gpioc_afrl_set(PIN_LEFT_MOTOR.1 * 4,4,AF2);

    }
    else{
        //gpioc_afrh_write(rep_bits(gpioc_afrh_read(),PIN_LEFT_MOTOR.1 * 4,4,AF2));
        gpioc_afrh_set(PIN_LEFT_MOTOR.1 * 4,4,AF2);

    }

    init_timer();

    //// Set direction to forward (right)
    //GPIOD_BSRR = 1 << PIN_RIGHT_FORWARD;
    digital_write(PIN_RIGHT_FORWARD,HIGH);
    //GPIOD_BSRR = 1 << (16 + PIN_RIGHT_BACKWARD);
    digital_write(PIN_RIGHT_BACKWARD,LOW);


    //// Set direction to forward (left)
    //GPIOD_BSRR = 1 << PIN_LEFT_FORWARD;
    digital_write(PIN_LEFT_FORWARD,HIGH);
    //GPIOD_BSRR = 1 << (16 + PIN_LEFT_BACKWARD);
    digital_write(PIN_LEFT_BACKWARD,LOW);

    
    loop {
        // getting fast
        for i in 0..=100{
            set_speed_right(i);
			set_speed_left(i);

        }
        // getting slow
        for i in (0..=100).rev() {
            set_speed_right(i);
            set_speed_left(i);
        }
    }
}