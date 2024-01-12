import sys



libname = sys.argv[1]
inputfile = sys.argv[2]


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