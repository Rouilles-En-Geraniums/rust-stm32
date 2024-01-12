mod our_library;


fn main() {
    
    //our_library::initRegister(GPIOA);

    our_library::initGPIO(GREEN_LED_BUILTIN, OUTPUT);

    while (1){
        our_library::digitalWrite(GREEN_LED_BUILTIN, HIGH)
        our_library::wait(500); // ??????????
        our_library::digitalWrite(GREEN_LED_BUILTIN, LOW)
        our_library::wait(500); // ??????????
        
    }
}

