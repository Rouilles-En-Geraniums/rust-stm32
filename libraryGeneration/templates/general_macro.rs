// Global variables


{% macro gen_addresses(component,address) -%}
const {{component}}_ADR : *mut u32 = {{address}} as *mut u32;
{%- endmacro %}



{% macro gen_register_offset(component, register, offset) -%}
const {{component}}_{{register}}_OFFSET : isize = {{offset}};
{%- endmacro %}



// Functions

{% macro gen_register_write(component, register) -%}
pub fn {{component.lower()}}_{{register.lower()}}_write(value: u32) {
    unsafe { *{{component}}_ADR.byte_offset({{component}}_{{register}}_OFFSET) = value};
}
{%- endmacro %}


{% macro gen_register_read(component, register) -%}
pub fn {{component.lower()}}_{{register.lower()}}_read() -> u32 {
    unsafe { * {{component}}_ADR.byte_offset({{component}}_{{register}}_OFFSET)}
}
{%- endmacro %}