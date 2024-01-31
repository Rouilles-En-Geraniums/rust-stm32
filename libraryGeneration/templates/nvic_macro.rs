{% macro gen_nvic_set(component, register) -%}
pub fn {{component.lower()}}_{{register.lower()}}_set(vector: u32, value: u32) {
    unsafe {
        write_volatile( ({{component}}_ADR + {{component}}_{{register}}_OFFSET + (vector >> 5) ) as *mut u32, 1 << (value & 0x1F) )
    };
}
{%- endmacro %}
