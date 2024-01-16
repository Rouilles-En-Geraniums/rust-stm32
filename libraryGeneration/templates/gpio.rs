{%- import "gpio_macro.rs" as gpiomacro %}


{%- for gpio in gpios %}
{{gpiomacro.gen_GPIO_ADR(gpio.name, gpio.address)}}
{%- endfor %}

{%- for gpio in gpios %}
    {% for register in registers %}
{{gpiomacro.gen_register_offset(gpio.name, register.name, register.offset)}}
    {%- endfor %}
{%- endfor %}

{% for gpio in gpios %}
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
        {{gpiomacro.gen_digital_write_switch_case(gpio.name[-1])}}
        {%- endfor %}
        _ => err,
    }
    
}
