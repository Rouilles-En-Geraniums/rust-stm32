// il faut mettre l'équivalent de "volatile" partout
const GPIO_ADR : u32 = $GPIO_ADR ; //0x40020000
const GPIO_size : u32 = $GPIO_size ; //0x400

const GPIOA_position : u8 = $GPIOA_position ; //0
//const GPIO$(letter)_position : u8 = $(GPIO$(letter)_position) ;

const BSRR_offset : u32 = $BSRR_offset  ; //0x18


fn GPIOA_BSRR_write(value: u8){
    GPIO_ADR + GPIO_size * GPIOA_position + BSRR_offset = value;
}



fn initGPIO(pin: (char,u8), mode: u8){
    //GPIOD_MODER = REP_BITS(GPIOD_MODER, (GREEN_LED)*2, 2, GPIO_MODER_OUT) ;
    //GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));
}

/**
 * pin = (GPIO : char, Pin : u8)
 * mode = HIGH/LOW
 */
fn digitalWrite(pin: (char,u8), mode: u8){
    // pin = (A, 2), mode = 1
    match pin.0 {
        A => {
            match mode{
                HIGH => GPIOA_BSRR_write(1 << pin.1),
                LOW => GPIOA_BSRR_write(1 << (pin.1 + 16)),
                _ => error,
            }
        },
        B => {

        },
        _ => err,
    }
    
}

