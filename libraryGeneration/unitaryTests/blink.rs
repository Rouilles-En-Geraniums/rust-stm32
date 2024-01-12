mod our_library; // ou 'use' ?


fn main() {
    
    //our_library::initRegister(GPIOA);

    let my_led = (GPIOA, 2);

    our_library::initGPIO(my_led, OUTPUT);

    while (1){
        our_library::digitalWrite(my_led, HIGH)
        our_library::wait(500); // ??????????
        our_library::digitalWrite(my_led, LOW)
        our_library::wait(500); // ??????????
        
    }
}

