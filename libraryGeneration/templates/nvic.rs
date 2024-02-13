{%- import "nvic_macro.rs" as nvicmacro %}
extern crate core;
use crate::core::ptr::write_volatile;

{%- include "address.rs" %}

{%- for component in components -%}
    {%- for register in registers %}
{{nvicmacro.gen_nvic_set(component.name, register.name)}}
    {%- endfor %}
{%- endfor -%}

pub fn nvic_handler_set(vector: u32, f: unsafe extern "C" fn()){
    unsafe {
        write_volatile( (NVIC_IRQ + vector * 4) as *mut u32, f as u32);
    }
}