{%- import "general_macro.rs" as gpiomacro %}

{%- for component in components %}
{{gpiomacro.gen_addresses(component.name, component.address)}}
{%- endfor %}

{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{gpiomacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{gpiomacro.gen_register_write(component.name, register.name)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_write(component.name, register.name)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{gpiomacro.gen_register_read(component.name, register.name)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}

