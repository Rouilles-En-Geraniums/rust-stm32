// il faut mettre l'équivalent de "volatile" partout

{%- macro gen_digital_write_switch_case(gpioName) -%}
        '{{gpioName}}' => match mode {
            HIGH => gpio{{gpioName.lower()}}_bsrr_write(1 << pin.1),
            LOW => gpio{{gpioName.lower()}}_bsrr_write(1 << (pin.1 + 16)),
            _ => (),
        },
{%- endmacro %}


{%- macro gen_digital_read_switch_case(gpioName) -%}
        '{{gpioName}}' => {
            if (gpio{{gpioName.lower()}}_idr_read() & (1 << pin.1)) != 0 {
                HIGH
            } else {
                LOW
            }
        }
{%- endmacro %}


{% macro gen_init_gpio_switch_case(gpioName) %}
    '{{gpioName}}' => {
        match types {
                MODER => match mode{
                    GPIO_MODER_IN => {
                        gpio{{gpioName.lower()}}_moder_write(rep_bits(gpio{{gpioName.lower()}}_moder_read(), pin.1*2, 2, GPIO_MODER_IN));
                                },
                    GPIO_MODER_OUT => {
                        gpio{{gpioName.lower()}}_moder_write(rep_bits(gpio{{gpioName.lower()}}_moder_read(), pin.1*2, 2, GPIO_MODER_OUT));
                    },
                    GPIO_MODER_ALT => {
                        gpio{{gpioName.lower()}}_moder_write(rep_bits(gpio{{gpioName.lower()}}_moder_read(), pin.1*2, 2, GPIO_MODER_ALT));
                    },
                    GPIO_MODER_ANA => {
                        gpio{{gpioName.lower()}}_moder_write(rep_bits(gpio{{gpioName.lower()}}_moder_read(), pin.1*2, 2, GPIO_MODER_ANA));
                    },
                    _ => (),
                }
                PUPDR => match mode {
                    GPIO_PUPDR_NO => {
                        gpio{{gpioName.lower()}}_pupdr_write(rep_bits(gpio{{gpioName.lower()}}_pupdr_read(), pin.1*2, 2, GPIO_PUPDR_NO));
                    },
                    GPIO_PUPDR_PU => {
                        gpio{{gpioName.lower()}}_pupdr_write(rep_bits(gpio{{gpioName.lower()}}_pupdr_read(), pin.1*2, 2, GPIO_PUPDR_PU));
                    },
                    GPIO_PUPDR_PD => {
                        gpio{{gpioName.lower()}}_pupdr_write(rep_bits(gpio{{gpioName.lower()}}_pupdr_read(), pin.1*2, 2, GPIO_PUPDR_PD));
                    },
                    _ => (),

                }
                OSPEEDER => match mode {
                    GPIO_OSPEEDR_LO => {
                        gpio{{gpioName.lower()}}_ospeedr_write(rep_bits(gpio{{gpioName.lower()}}_ospeedr_read(), pin.1*2, 2, GPIO_OSPEEDR_LO));
                    },
                    GPIO_OSPEEDR_ME => {
                        gpio{{gpioName.lower()}}_ospeedr_write(rep_bits(gpio{{gpioName.lower()}}_ospeedr_read(), pin.1*2, 2, GPIO_OSPEEDR_ME));
                    },
                    GPIO_OSPEEDR_HI => {
                        gpio{{gpioName.lower()}}_ospeedr_write(rep_bits(gpio{{gpioName.lower()}}_ospeedr_read(), pin.1*2, 2, GPIO_OSPEEDR_HI));
                    },
                    GPIO_OSPEEDR_VH => {
                        gpio{{gpioName.lower()}}_ospeedr_write(rep_bits(gpio{{gpioName.lower()}}_ospeedr_read(), pin.1*2, 2, GPIO_OSPEEDR_VH));
                    },
                    _ => (),
                }
                _ =>(),
        }
    },
{% endmacro %}


