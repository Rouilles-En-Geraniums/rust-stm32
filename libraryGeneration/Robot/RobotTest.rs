#![no_std]
#![no_main]

extern crate core ; 
extern crate geranium_rt;

use geranium_rt::stm32rustlib::gpio::*;
use geranium_rt::stm32rustlib::rcc::*;
use geranium_rt::stm32rustlib::tim::*;
use geranium_rt::stm32rustlib::delay::*;
use geranium_rt::stm32rustlib::various::*;
use geranium_rt::{print, println};


const T3_PSC: u32 = 14;
const P3_20MS: u32 = (42 * 1000000) / 14; // TODO : define APB1_CLK
const PSERVO : u32 = P3_20MS;
/****************
     * DEFINE QTR8 AND MOTOR PINS  *
    *****************/
const PIN_RIGHT_FORWARD: (char,u32) = ('D', 1);
const PIN_RIGHT_BACKWARD: (char,u32) = ('D', 0);
const PIN_RIGHT_MOTOR: (char,u32) = ('C', 6);
const PIN_LEFT_FORWARD: (char,u32) = ('D', 3);
const PIN_LEFT_BACKWARD: (char,u32) = ('D', 2);
const PIN_LEFT_MOTOR: (char,u32) = ('C', 7);
const AF2: u32 = 2;

const QTR8_PIN0: (char,u32) = ('D', 11);
const QTR8_PIN1: (char,u32) = ('D', 9);
const QTR8_PIN2: (char,u32) = ('D', 7);
const QTR8_PIN3: (char,u32) = ('D', 5);
const QTR8_PIN4: (char,u32) = ('D', 4);
const QTR8_PIN5: (char,u32) = ('D', 6);
const QTR8_PIN6: (char,u32) = ('D', 8);
const QTR8_PIN7: (char,u32) = ('D', 10);

const GPIO_QTR8_LED: (char,u32) = ('C', 15);
const QTR8_PINS: [(char,u32); 8] = [QTR8_PIN0, QTR8_PIN1, QTR8_PIN2, QTR8_PIN3, QTR8_PIN4, QTR8_PIN5, QTR8_PIN6, QTR8_PIN7];
const QTR8_NUM_PINS: usize = 8;
const num : usize = 10;

//----------------------------------
const MAX_SPEED:u32 = 256;
/****************
     * PID CONST  *
    *****************/
const KP: f32 = 0.002;//set up the constants value
const KI: f32 = 0.001;
const KD: f32 = 15.0 ; 
const KR: f32 =  0.0;
static mut P : u32 = 0 ;
static mut I : u32 = 0 ;
static mut D : u32 = 0 ;
static mut R : u32 = 0 ;

/****************
     * STATIC VARS  *
    *****************/
static mut ACTIVES: u32 = 0;
static mut LAST_IDLE: u32 = 0;// 0 -> Left, 1 -> Right 
static mut LAST_END: u32 = 0;
static mut LAST_ERROR: u32 = 0;
static mut ERROR_SUM: u32 = 0;
static mut SENSOR_READ : u32 = 0x00000000;
static mut POSITION: u32 = 0;
static mut ERRORS: [u32;10] = [0,0,0,0,0,0,0,0,0,0];
static mut ACTIVE: u32 = 0;

const maxspeedr  : u32= 100;
const maxspeedl  : u32= 100;
const basespeedr : u32 = 92;
const basespeedl : u32 = 92;

/****************
     * TIMER  *
    *****************/
fn init_timer(){
    tim3_cr1_write(0); 
    tim3_psc_write(T3_PSC-1);
    tim3_arr_write(PSERVO);
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC1M_PWM1); 
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_CCS1S_OUT); 
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC1PE); 
    tim3_ccer_seti(TIM_CC1E);
    tim3_ccer_write(tim3_ccer_read() & !TIM_CC1P); 
    tim3_ccr1_write(0); 
    

    // Left Motor
    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_OC2M_PWM1);

    tim3_ccmr1_write(tim3_ccmr1_read() | TIM_CCS2S_OUT);

	// we change the value of CCR2 so it's cleaner to have it prealoaded
    tim3_ccmr1_seti(TIM_OC2PE);


	//TIM3_CCER |= TIM_CC2E; // OC2 signal is output on the corresponding output pin
    tim3_ccer_seti(TIM_CC2E);
    tim3_ccer_seti( !TIM_CC2P);
    tim3_ccr2_write(0); 


	// Generate an event to store the value from preload register

    tim3_egr_write(TIM_UG);

    tim3_sr_write(0); 
	// turn on timer
    tim3_cr1_write(TIM_CEN);
}


/****************
     * MOTOR  *
    *****************/
// speed must be in [0, 100]
fn set_speed_right(speed: u32) {
    tim3_ccr1_write(speed * PSERVO / 100 );
}
// speed must be in [0, 100]
fn set_speed_left(speed: u32) {
    tim3_ccr2_write(speed * PSERVO / 100 );
}

fn set_speed_ml_positive(x: u32){
    set_speed_left(x);
    digital_write(PIN_LEFT_FORWARD,HIGH);
    digital_write(PIN_LEFT_BACKWARD,LOW);

}
fn set_speed_mr_positive(x: u32){
    set_speed_right(x);
    digital_write(PIN_RIGHT_FORWARD,HIGH);
    digital_write(PIN_RIGHT_BACKWARD,LOW);
}


fn set_speed_ml_negative(x: u32){
    set_speed_left(x);
    digital_write(PIN_LEFT_FORWARD,LOW);
    digital_write(PIN_LEFT_BACKWARD,HIGH);
}
fn set_speed_mr_negative(x: u32){
    set_speed_right(x);
    digital_write(PIN_RIGHT_FORWARD,LOW);
    digital_write(PIN_RIGHT_BACKWARD,HIGH);
}




fn turn_left(x: u32){
    set_speed_ml_positive(0);
    set_speed_mr_positive(x);
}

fn turn_right(x: u32){
    set_speed_mr_positive(0);
	set_speed_ml_positive(x);
}

fn motor_control(x_right: u32 , x_left: u32){
    turn_left(x_left) ; 
    turn_right(x_right);
}


fn sharp_turn() {
	
	if unsafe {LAST_IDLE < 25}  
	{
		if unsafe {LAST_END == 1} {
			motor_control(0, 100);
        }
		else{
			motor_control(100, 0);
        }
	}
	else 
	{
		if unsafe {LAST_END == 1}{
			motor_control(0, 70);
        }
		else{
			motor_control(70, 0);
        }
    }
    
}


fn forward_brake(x_right: u32 , x_left: u32){
	if unsafe {ACTIVES == 0}{ 
		sharp_turn();
    }
	else{
        motor_control(x_right, x_left);

    }
}
/****************
     * QTR  *
    *****************/
fn set_qtr8_input(){
    for i in 0..=QTR8_NUM_PINS-1 {
        gpiod_moder_set(QTR8_PINS[i].1*2,2,GPIO_MODER_IN);
        gpiod_pupdr_set(QTR8_PINS[i].1*2,2,GPIO_PUPDR_PD);
    }
}

fn set_qtr8_output(){
    for i in 0..=QTR8_NUM_PINS-1{
        gpiod_moder_set(QTR8_PINS[i].1*2,2,GPIO_MODER_OUT);
    }
}


fn set_qtr8_high(){
    for i in 0..=QTR8_NUM_PINS-1{
        digital_write(QTR8_PINS[i],HIGH);
    }
}

fn qtr8_read_sensor(pin : (char, u32))-> bool{
    digital_read(pin) == HIGH
}

fn turn_on_qtr8_led(){ digital_write(GPIO_QTR8_LED,HIGH);}
fn turn_off_qtr8_led(){ digital_write(GPIO_QTR8_LED,LOW);}

fn init_qtr8(){
    gpioc_moder_set(GPIO_QTR8_LED.1*2,2,GPIO_MODER_OUT);
    turn_off_qtr8_led();
    set_qtr8_output(); 
}

fn qtr8_read() -> u32 {
    turn_on_qtr8_led();
    set_qtr8_output();
    set_qtr8_high();
    delay_us(12);
    set_qtr8_input();
    // Threshold
    unsafe {SENSOR_READ = 0x00000000 ;} 
    unsafe {ACTIVE = 0 ;} 
    unsafe {POSITION = 0;}
    if digital_read(QTR8_PIN0) == HIGH {
        unsafe  {SENSOR_READ |= 0x00000001;}
        unsafe  {POSITION +=1000 ;}
        unsafe  {ACTIVE +=1 ;}
        unsafe  {LAST_END = 1 ;}
    }
    if digital_read(QTR8_PIN1) == HIGH {
        unsafe  {SENSOR_READ |= 0x00000010;}
        unsafe  {POSITION +=22000 ;}
        unsafe  {ACTIVE +=1 ;}

    }
    if digital_read(QTR8_PIN2) == HIGH {
        unsafe  {SENSOR_READ |= 0x00000100;}
        unsafe  {POSITION +=3000 ;}
        unsafe  {ACTIVE +=1 ;}
    }
    if digital_read(QTR8_PIN3) == HIGH {
        unsafe  {SENSOR_READ |= 0x00001000;}
        unsafe  {POSITION +=4000 ;}
        unsafe  {ACTIVE +=1 ;}
    }
    if digital_read(QTR8_PIN4) == HIGH {
        unsafe  {SENSOR_READ |= 0x00010000;}
        unsafe  {POSITION +=5000 ;}
        unsafe  {ACTIVE +=1 ;}
    }
    if digital_read(QTR8_PIN5) == HIGH {
        unsafe  {SENSOR_READ |= 0x00100000;}
        unsafe  {POSITION +=6000 ;}
        unsafe  {ACTIVE +=1 ;}
    }
    if digital_read(QTR8_PIN6) == HIGH {
        unsafe  {SENSOR_READ |= 0x01000000;}
        unsafe  {POSITION +=7000 ;}
        unsafe  {ACTIVE +=1 ;}
    }
    if digital_read(QTR8_PIN7) == HIGH {
        unsafe  {SENSOR_READ |= 0x10000000;}
        unsafe  {POSITION +=8000 ;}
        unsafe  {ACTIVE +=1 ;}
        unsafe  {LAST_END = 0 ;}
    }
    turn_off_qtr8_led(); 
    unsafe {ACTIVES = ACTIVE ; }
    unsafe {POSITION = POSITION/ACTIVE ; }
    if unsafe {ACTIVES == 0}{
        unsafe {LAST_IDLE +=1 ;}
    } 
    else{ 
        unsafe {LAST_IDLE = 0}
    }
    if unsafe{ACTIVE != 0} {
        unsafe{POSITION /= ACTIVE};
    }
    unsafe{POSITION = POSITION};

    return unsafe{POSITION};
}

fn past_errors (error: u32) 
{
  for i in (0..9).rev(){
    unsafe{ERRORS[i] = ERRORS[i-1]};
  }
  unsafe {ERRORS[0] = error};
}

fn errors_sum (index : u32,abs : u32) -> u32 
{
  //int sum = 0;
  static mut sum: u32 = 0; 
  for i in 0..(index as usize){
    if abs == 1{
        unsafe{sum -=  ERRORS[i]};
    }
    else{
        unsafe{sum +=  ERRORS[i]};
    }
  }
  return unsafe{sum};
}

//void PID_control() {
//	uint16_t position = QTR8_read();	
//  int error = 4500 - position;
//	past_errors(error);
//
//  P = error;
//  I = errors_sum(5, 0);
//  D = error - lastError;
//  R = errors_sum(5, 1);
//  lastError = error;
//	
//  int motorspeed = P*Kp + I*Ki + D*Kd;
//  
//  int motorspeedl = basespeedl + motorspeed - R*Kr;
//  int motorspeedr = basespeedr - motorspeed - R*Kr;
//  
//  if (motorspeedl > maxspeedl)
//    motorspeedl = maxspeedl;
//  if (motorspeedr > maxspeedr)
//    motorspeedr = maxspeedr;
//	
//	forward_brake(motorspeedr, motorspeedl);
//}

fn PID_control(){
    unsafe { POSITION = qtr8_read() ; }
    let error :u32 = 4500- unsafe{POSITION} ; 
    past_errors(error);
    unsafe { P = error;}
    unsafe { I = errors_sum(5, 0);}
    unsafe { D = error - LAST_ERROR;}
    unsafe { R = errors_sum(5, 1);}
    unsafe { LAST_ERROR = error}

    let motorspeed : u32 = unsafe{P*(KD as u32) + I*(KI as u32) + D*(KD as u32)};
    static mut  motorspeedl  :u32 = 0 ; 
    unsafe{motorspeedl = basespeedl + motorspeed - R*(KR as u32)};
    static mut  motorspeedr  :u32 = 0 ; 
    unsafe{motorspeedr = basespeedr - motorspeed - R*(KR as u32)};
    if unsafe{motorspeedl > maxspeedl}{
        unsafe{motorspeedl = maxspeedl;}
    }
     if unsafe{motorspeedr > maxspeedr}{
        unsafe{motorspeedr = maxspeedr;}
    }
	
	forward_brake(unsafe{motorspeedr}, unsafe{motorspeedl});

}
// init all pins
fn init_enable_pins(){

/****************
     * RIGHT MOTOR *
    *****************/
    // configure directional control pins for right motor 
    gpiod_moder_set(PIN_RIGHT_FORWARD.1*2,2,GPIO_MODER_OUT);
    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_RIGHT_BACKWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_RIGHT_BACKWARD.1*2,2,GPIO_MODER_OUT);

    // set alternative mode
    gpioc_moder_set(PIN_RIGHT_MOTOR.1*2,2,GPIO_MODER_ALT);

    if PIN_RIGHT_MOTOR.1 <= 7 {
        gpioc_afrl_set(PIN_RIGHT_MOTOR.1 * 4,4,AF2);
    }
    else{
        gpioc_afrh_set(PIN_RIGHT_MOTOR.1 * 4,4,AF2);

    }

    /***************
     * LEFT MOTOR *
    ****************/


    // configure directional control pins for left motor
    gpiod_moder_set(PIN_LEFT_FORWARD.1*2,2,GPIO_MODER_OUT);

    //gpiod_moder_write(rep_bits(gpiod_moder_read(), PIN_LEFT_BACKWARD.1 * 2, 2, GPIO_MODER_OUT));
    gpiod_moder_set(PIN_LEFT_BACKWARD.1*2,2,GPIO_MODER_OUT);

    // set alternative modeunsafe{2,2,GPIO_MODER_ALT);

    if PIN_LEFT_MOTOR.1 <= 7 {
        gpioc_afrl_set(PIN_LEFT_MOTOR.1 * 4,4,AF2);

    }
    else{
        gpioc_afrh_set(PIN_LEFT_MOTOR.1 * 4,4,AF2);

    }

}


#[no_mangle]
fn main() {
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOAEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIODEN);
    rcc_ahb1enr_seti(RCC_AHB1ENR_GPIOCEN);
    rcc_apb1enr_seti(RCC_APB1ENR_TIM3EN);
    delay_init_timers();


    init_enable_pins();
    init_timer();
    println!("tst");
    //motor_control(0,50);
   //motor_control(50,50);
    loop{
        
        println!("value :{}",qtr8_read());
    }
}

