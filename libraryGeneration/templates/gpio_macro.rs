// il faut mettre l'équivalent de "volatile" partout

// Global variables

{% macro gen_GPIO_ADR(gpioAdr) -%}
const GPIO_ADR : u32 = {{gpioAdr}} ; //0x40020000
{%- endmacro %}

{% macro gen_GPIO_size(gpioSize) -%}
const GPIO_size : u32 = {{gpioSize}} ; //0x400
{%- endmacro %}

{% macro gen_GPIO_position(gpioName, gpioPosition) -%}
const GPIO{{gpioName}}_position : u8 = {{gpioPosition}} ;
{%- endmacro %}

{% macro gen_register_offset(registerName, registerOffset) -%}
const {{registerName}}_offset : u32 = {{registerOffset}} ; //0x18
{%- endmacro %}

{% macro gen_GPIO_write(gpioName, registerName) -%}
fn GPIO{{gpioName}}_{{registerName}}_write(value: u8){
    GPIO_ADR + GPIO_size * GPIO{{gpioName}}_position + {{registerName}}_offset = value;
}
{%- endmacro %}

{%- macro gen_digital_write_switch_case(gpioName) -%}
        {{gpioName}} => {
            match mode {
                HIGH => GPIO{{gpioName}}_BSRR_write(1 << pin.1),
                LOW => GPIO{{gpioName}}_BSRR_write(1 << (pin.1 + 16)),
                _ => error,
            }
        },
{%- endmacro %}

{% macro f(a, b) -%}
{%- endmacro %}









