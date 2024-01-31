{%- import "nvic_macro.rs" as nvicmacro %}
extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;

{%- include "address.rs" %}

{%- for component in components -%}
    {%- for register in registers -%}
{{nvicmacro.gen_nvic_set(component.name, register.name)}}
    {%- endfor -%}
{%- endfor -%}