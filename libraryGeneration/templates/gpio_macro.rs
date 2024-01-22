// il faut mettre l'équivalent de "volatile" partout

{%- macro gen_digital_write_switch_case(gpioName) -%}
        {{gpioName}} => {
            match mode {
                HIGH => gpio{{gpioName.lower()}}_bsrr_write(1 << pin.1),
                LOW => gpio{{gpioName.lower()}}_bsrr_write(1 << (pin.1 + 16)),
                _ => error,
            }
        },
{%- endmacro %}

{% macro f(a, b) -%}
{%- endmacro %}









