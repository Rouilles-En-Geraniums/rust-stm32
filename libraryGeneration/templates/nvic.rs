{%- import "nvic_macro.rs" as nvicmacro %}
extern crate core;
use crate::core::ptr::write_volatile;
use crate::core::ptr::read_volatile;
use crate::stm32rustlib::various;

{%- include "address.rs" %}

{%- for component in components -%}
    {%- for register in registers %}
{{nvicmacro.gen_nvic_set(component.name, register.name)}}
    {%- endfor %}
{%- endfor -%}

{%- for constant in constants %}
    {%- if constant.name == "NVIC_IRQ" %}
pub fn nvic_handler_set(vector: u32, f: u32){
    unsafe {
        write_volatile(({{constant.value}} + vector * 4) as *mut u32, f);
    }
}
    {%- endif -%}
{%- endfor %}