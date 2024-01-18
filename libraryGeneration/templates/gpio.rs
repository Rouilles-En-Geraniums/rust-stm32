{%- import "gpio_macro.rs" as gpiomacro %}


{%- for component in components %}
{{gpiomacro.gen_addresses(component.name, component.address)}}
{%- endfor %}

{%- for component in components %}
    {%- if exhaustive %}
        {% for register in components.registers %}
{{gpiomacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in components.registers %}
{{gpiomacro.gen_register_write(component.name, register.name)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_write(component.name, register.name)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in components.registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% endif %}
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
        {%- for component in components %}
        {{gpiomacro.gen_digital_write_switch_case(component.name[-1])}}
        {%- endfor %}
        _ => err,
    }
    
}
