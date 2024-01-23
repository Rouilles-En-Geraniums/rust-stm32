{%- import "gpio_macro.rs" as gpiomacro %}

use crate::stm32rustlib::various;
{% include "address.rs" %}

/*
fn initGPIO(pin: (char,u8), mode: u8){
    GPIOD_MODER = REP_BITS(GPIOD_MODER, (GREEN_LED)*2, 2, GPIO_MODER_OUT) ;
    GPIOD_OTYPER = GPIOD_OTYPER & ~( 1 << (GREEN_LED));
}
*/

/**
 * pin = (GPIO : char, Pin : u8)
 * mode = HIGH/LOW
 */
pub fn digital_write(pin: (char,u8), mode: u8){
    // pin = (A, 2), mode = 1
    match pin.0 {
        {%- for component in components %}
        {{gpiomacro.gen_digital_write_switch_case(component.name[-1])}}
        {%- endfor %}
        _ => (),
    }
    
}
