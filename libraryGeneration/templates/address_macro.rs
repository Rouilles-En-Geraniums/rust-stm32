// Global variables


{% macro gen_addresses(component,address) -%}
const {{component}}_ADR: u32 = {{address}};
{%- endmacro %}



{% macro gen_register_offset(component, register, offset) -%}
const {{component}}_{{register}}_OFFSET: u32 = {{offset}};
{%- endmacro %}


{% macro gen_constants(name, value) -%}
pub const {{name}}: u32 = {{value}};
{%- endmacro %}



// Functions

{% macro gen_register_write(component, register) -%}
#[inline(always)]
pub fn {{component.lower()}}_{{register.lower()}}_write(value: u32) {
    unsafe { write_volatile(({{component}}_ADR + {{component}}_{{register}}_OFFSET) as *mut u32, value) };
}
{%- endmacro %}


{% macro gen_register_read(component, register) -%}
#[inline(always)]
pub fn {{component.lower()}}_{{register.lower()}}_read() -> u32 {
    unsafe { read_volatile(({{component}}_ADR + {{component}}_{{register}}_OFFSET) as *mut u32) }
}
{%- endmacro %}


{% macro gen_register_set(component, register) -%}
#[inline(always)]
pub fn {{component.lower()}}_{{register.lower()}}_set(position: u32, size: u32, value: u32) {
    {{component.lower()}}_{{register.lower()}}_write(rep_bits({{component.lower()}}_{{register.lower()}}_read(), position, size, value));
}
{%- endmacro %}

{% macro gen_register_seti(component, register) -%}
#[inline(always)]
pub fn {{component.lower()}}_{{register.lower()}}_seti(value: u32) {
    match value.count_ones() {
        1 => {{component.lower()}}_{{register.lower()}}_write({{component.lower()}}_{{register.lower()}}_read() | value),
        31 => {{component.lower()}}_{{register.lower()}}_write({{component.lower()}}_{{register.lower()}}_read() & value),
        _ => (),
    }
}
{%- endmacro %}