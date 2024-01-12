//TODO : built-in LEDs should be instanciated upon library generation
// GPIOD
//const GREEN_LED: u8 = 12;
//const GPIOA: u8 = adresse du GPIO A (=GPIO 1)



fn initRegister(name: u8){
    //RCC_AHB1ENR |= RCC_GPIODEN;
}

fn wait(t: u32){
    // fonction blocante
    // t en ms

    //const ONE_SECOND: u32 = 30000000
    let n = t/1000 * ONE_SECOND;

    for i in 0..n {
        NOP; // TODO ? maybe it is a macro
    }
} 