import sys
from jinja2 import Environment, FileSystemLoader

import json

def generate_data_from_json(json_data):
    data = {
        "gpios": [{"name": gpio_name, "address": index} for index, (gpio_name, _) in enumerate(json_data.items()) if gpio_name.startswith("GPIO")],
        "registers": {gpio_name: [{"name": register["name"], "offset": register["offset"], "read": register["read"], "write": register["write"]} for register in gpio_data.get("registers", [])] for gpio_name, gpio_data in json_data.items() if gpio_name.startswith("GPIO")}
    }
    return data

with open('../descriptionFiles/gpio.json', 'r') as json_file:
    json_data = json.load(json_file)

data = generate_data_from_json(json_data)



file_loader = FileSystemLoader('../templates/')
env = Environment(loader=file_loader)
t = env.get_template("gpio.rs")

print(t.render(data))



#libname = sys.argv[1]
#inputfile = sys.argv[2]


'''
créer un dossier $libname dans ../tests/ et y met tous les
fichiers de libraire rust généré

idéalement : un fichier par "type de registre", ex : 
- gpio.rs
- nvic.rs
- adc.rs
- rcc.rs 
- etc...


Dans ce fichier python :
- une première section pour charger le fichier de description
- déterminer quelles sections sont à générer
- une section "généraliste" 
- une section par "type de registre"
- commencer avec uniquement une section pour le gpio (PoC)
- s'inspirer de https://github.com/stm32-rs/stm32f4xx-hal pour
    avoir une idée de quelles sections faire par la suite
'''





## Section Init

'''
importer le fichier de description
variables etc..
'''

## Section généraliste

'''
initRegister()
wait()
...
'''

## Section GPIO

'''
initGPIO()
digitalWrite()
...
'''

## Section ?


## Section "fin"