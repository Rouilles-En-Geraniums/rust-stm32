// Global variables


{% macro gen_addresses(component,address) -%}
const {{component}}_ADR : u32 = {{address}};
{%- endmacro %}



{% macro gen_register_offset(component, register, offset) -%}
const {{component}}_{{register}}_OFFSET : u32 = {{offset}};
{%- endmacro %}



// Functions

{% macro gen_register_write(component, register) -%}
pub fn {{component.lower()}}_{{register.lower()}}_write(value: u32) {
    unsafe {
        write_volatile( ({{component}}_ADR + {{component}}_{{register}}_OFFSET) as *mut u32, value)
    };
}
{%- endmacro %}


{% macro gen_register_read(component, register) -%}
pub fn {{component.lower()}}_{{register.lower()}}_read() -> u32 {
    unsafe {
        read_volatile( ({{component}}_ADR + {{component}}_{{register}}_OFFSET) as *mut u32)
    }
}
{%- endmacro %}