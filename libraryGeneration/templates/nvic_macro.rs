{% macro gen_nvic_set(component, register) -%}
#[inline(always)]
pub fn {{component.lower()}}_{{register.lower()}}_set(vector: u32, value: u32) {
    unsafe {
        write_volatile(
            ({{component}}_ADR + {{component}}_{{register}}_OFFSET + (vector) * 4) as *mut u32,
            value,
        )
    };
}
{%- endmacro %}
