{%- import "general_macro.rs" as gpiomacro %}
const HIGH: u8 = 1;
const LOW: u8 = 0;

{%- for component in components %}
{{gpiomacro.gen_addresses(component.name, component.address)}}
{%- endfor %}

{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
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
        {% for register in component.registers %}
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
        {% for register in component.registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}




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