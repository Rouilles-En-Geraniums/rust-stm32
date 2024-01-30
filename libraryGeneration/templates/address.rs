{%- import "address_macro.rs" as generalmacro %}

{%- for component in components %}
{{generalmacro.gen_addresses(component.name, component.address)}}
{%- endfor %}

{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{generalmacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{generalmacro.gen_register_offset(component.name, register.name, register.offset)}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{generalmacro.gen_register_write(component.name, register.name) if register.write == 1 else ""}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{generalmacro.gen_register_write(component.name, register.name) if register.write == 1 else ""}}
        {%- endfor %}
    {% endif %}
{%- endfor %}


{%- for component in components %}
    {%- if exhaustive %}
        {% for register in component.registers %}
{{generalmacro.gen_register_read(component.name, register.name) if register.read == 1 else ""}}
        {%- endfor %}
    {% else %}
        {% for register in registers %}
{{generalmacro.gen_register_read(component.name, register.name) if register.read == 1 else ""}}
        {%- endfor %}
    {% endif %}
{%- endfor %}

