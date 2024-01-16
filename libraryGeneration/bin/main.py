import sys
from jinja2 import Environment, FileSystemLoader


data = {
    "gpios" : [
        {"name" : "GPIOA", "address" : 0},
        {"name" : "GPIOB", "address" : 1},
        {"name" : "GPIOC", "address" : 2},
        {"name" : "GPIOD", "address" : 3},
        {"name" : "GPIOE", "address" : 4},
        {"name" : "GPIOF", "address" : 5},
        {"name" : "GPIOG", "address" : 6},
        {"name" : "GPIOH", "address" : 7}
    ],
    "registers" : [
        {"name" : "IDR", "offset" : "0x10", "read" : 1, "write" : 0},
        {"name" : "ODR", "offset" : "0x14", "read" : 1, "write" : 1},
        {"name" : "BSRR", "offset" : "0x18", "read" : 0, "write" : 1}
    ]
}

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