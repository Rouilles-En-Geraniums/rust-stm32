// Global variables


{% macro gen_addresses(component,address) -%}
const {{component}}_ADR : u32 = {{address}} ;
{%- endmacro %}



{% macro gen_register_offset(component, register, offset) -%}
const {{component}}_{{register}}_offset : u32 = {{offset}} ; //0x18
{%- endmacro %}



// Functions

{% macro gen_register_write(component, register) -%}
fn {{component}}_{{register}}_write(value: u8) {
    {{component}}_ADR + {{component}}_{{register}}_offset = value;
}
{%- endmacro %}


{% macro gen_register_read(component, register) -%}
fn {{component}}_{{register}}_read(value: u8) {
    {{component}}_ADR + {{component}}_{{register}}_offset = value;
}
{%- endmacro %}