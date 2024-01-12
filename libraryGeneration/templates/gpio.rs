

fn initGPIO(pin: u8, mode: u8){
    //GPIOD_MODER = REP_BITS(GPIOD_MODER, (GREEN_LED)*2, 2, GPIO_MODER_OUT) ;
    //GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));
}

fn digitalWrite(pin: u8, mode: bool){
    match mode{
        HIGH => $sethigh(pin), //GPIOD_BSRR = 1 << GREEN_LED;
        LOW => $setlow(pin), //GPIOD_BSRR = 1 << (GREEN_LED + 16);
    }
}

