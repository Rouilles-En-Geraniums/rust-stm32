import sys
from jinja2 import Environment, FileSystemLoader


import json


def generate_data_from_json(json_data):

    if json_data["exhaustive"] : 
        
        data = {
            "exhaustive": 1,
            "components" : [{"name": component["name"],"address": component["address"],
                            "registers": [{"name": register["name"], "offset": register["offset"], "read": register["read"], "write": register["write"]} 
                            for register in json_data["registers"]]} for component in json_data["components"] ],
        }
    else :
        data = {
            "exhaustive": 0,
            "components" : [{"name": component["name"],"address": component["address"]} for component in json_data["components"] ],
            "registers": [{"name": register["name"], "offset": register["offset"], "read": register["read"], "write": register["write"]} for register in json_data["registers"]]
        }

    return data

if len(sys.argv) != 2:
    print("Usage: python script.py <json_file_path>")
    sys.exit(1)

json_file_path = sys.argv[1]

try:
    with open(json_file_path, 'r') as json_file:
        json_data = json.load(json_file)
except FileNotFoundError:
    print(f"Error: File '{json_file_path}' not found.")
    sys.exit(1)
except json.JSONDecodeError:
    print(f"Error: Unable to parse JSON in file '{json_file_path}'.")
    sys.exit(1)

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