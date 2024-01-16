// il faut mettre l'équivalent de "volatile" partout

// Global variables

{% macro gen_GPIO_ADR(gpioName,gpioAdr) -%}
const {{gpioName}}_ADR : u32 = {{gpioAdr}} ; //0x40020000
{%- endmacro %}

{% macro gen_register_offset(gpioName, registerName, registerOffset) -%}
const {{gpioName}}_{{registerName}}_offset : u32 = {{registerOffset}} ; //0x18
{%- endmacro %}

{% macro gen_GPIO_write(gpioName, registerName) -%}
fn {{gpioName}}_{{registerName}}_write(value: u8){
    {{gpioName}}_ADR + {{gpioName}}_{{registerName}}_offset = value;
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









