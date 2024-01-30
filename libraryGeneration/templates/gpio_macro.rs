// il faut mettre l'équivalent de "volatile" partout

{%- macro gen_digital_write_switch_case(gpioName) -%}
        '{{gpioName}}' => {
            match mode {
                various::HIGH => gpio{{gpioName.lower()}}_bsrr_write(1 << pin.1),
                various::LOW => gpio{{gpioName.lower()}}_bsrr_write(1 << (pin.1 + 16)),
                _ => (),
            }
        },
{%- endmacro %}


{%- macro gen_digital_read_switch_case(gpioName) -%}
        '{{gpioName}}' => {
            if (gpio{{gpioName.lower()}}_idr_read() & (1 << pin.1)) != 0 {
                various::HIGH
            } else {
                various::LOW
            }
        },
{%- endmacro %}









