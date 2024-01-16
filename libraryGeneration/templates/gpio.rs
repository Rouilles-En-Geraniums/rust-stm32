{%- import "gpio_macro.rs" as gpiomacro %}

{{gpiomacro.gen_GPIO_ADR(gpioAdr)}}

{{gpiomacro.gen_GPIO_size(gpioSize)}}

{%- for gpio in gpios %}
{{gpiomacro.gen_GPIO_position(gpio.name, gpio.position)}}
{%- endfor %}

{%- for register in registers %}
{{gpiomacro.gen_register_offset(register.name, register.offset)}}
{%- endfor %}

{%- for gpio in gpios %}
    {%- for register in registers %}
{{gpiomacro.gen_GPIO_write(gpio.name, register.name)}}
    {%- endfor %}
{%- endfor %}


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
        {%- for gpio in gpios %}
        {{gpiomacro.gen_digital_write_switch_case(gpio.name)}}
        {%- endfor %}
        _ => err,
    }
    
}
