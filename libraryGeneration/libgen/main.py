####
#	Rust on STM32 Project by Rouilles en GeraniumTM
#	Copyright (C) 2024 Université de Toulouse :
#   - Oussama Felfel - oussama.felfel@univ-tlse3.fr		
#   - François Foltete - francois.foltete@univ-tlse3.fr		
#   - Elana Courtines - elana.courtines@univ-tlse3.fr		
#   - Teo Tinarrage - teo.tinarrage@univ-tlse3.fr		
#   - Zineb Moubarik - zineb.moubarik@univ-tlse3.fr 
#
#  This library aims to provide the following :
#   - a rust library generation tool to safely access memory ;
#   - a support to flash STM32 boards ;
#   - a task scheduling tool that generates the associated rust code.
# 
#  The development of this library has done as a Proof of Concept and
#  is currently only tested for STM32F407-G DISC1 Boards.
# 
#  It is our hope that using this library to enable development on
#  other boards will be facilitated.
# 
# 
#	This program is free software: you can redistribute it and/or modify
#	it under the terms of the GNU General Public License as published by
#	the Free Software Foundation, either version 3 of the License, or
#	(at your option) any later version.
#
#	This program is distributed in the hope that it will be useful,
#	but WITHOUT ANY WARRANTY; without even the implied warranty of
#	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#	GNU General Public License for more details.
####


import sys
import os
import json
from jinja2 import Environment, FileSystemLoader
from pathlib import Path
import argparse
import shutil


def cmdlineParse():
    # Argument line parser initialisation.
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )

    # Parser argument control section.
    parser.add_argument("-o", "--outputdir",
                        help="Output directory.",
                        default="test")
    parser.add_argument("-l", "--libraryname",
                        help="Library name.",
                        default="stm32rustlib")
    parser.add_argument("-j", "--json",
                        help="JSON files to parse",
                        nargs='*',
                        default=[])
    parser.add_argument("-e", "--extra",
                        help="Additionnal files to be included in the library. Example: wait.rs",
                        nargs='*',
                        default=[])

    args = parser.parse_args()

    return args


def generate_data_from_json(json_file_path):
    try:
        with open(json_file_path, 'r') as json_file:
            json_data = json.load(json_file)
    except FileNotFoundError:
        print(f"Error: File '{json_file_path}' not found.")
        sys.exit(1)
    except json.JSONDecodeError:
        print(f"Error: Unable to parse JSON in file '{json_file_path}'.")
        sys.exit(2)

    if json_data["exhaustive"]:
        data = {
            "exhaustive": 1,
            "components": [{"name": component["name"],
                            "address": component["address"],
                            "registers": [{"name": register["name"],
                                           "offset": register["offset"],
                                           "read": register["read"],
                                           "write": register["write"]}
                                           for register in component["registers"]]}
                            for component in json_data["components"]],
            "constants" : [{"name": constant["name"],    
                            "value": constant["value"] }
                               for  constant in json_data["constants"]]
            }
    else:
        data = {
            "exhaustive": 0,
            "components": [{"name": component["name"],
                            "address": component["address"]}
                            for component in json_data["components"]],
            "registers": [{"name": register["name"],
                           "offset": register["offset"],
                           "read": register["read"],
                           "write": register["write"]}
                           for register in json_data["registers"]],
            "constants" : [{"name": constant["name"],    
                            "value": constant["value"] }
                               for  constant in json_data["constants"]]
             }


    return data


def main():

    # Parse argument line
    args = cmdlineParse()

    output_dir_path = args.outputdir
    library_name = args.libraryname
    library_dir_path = os.path.join(output_dir_path,library_name)
    json_files = args.json
    extra_files = args.extra

    # Initiate Jinja2 environment
    file_loader = FileSystemLoader('../templates/')
    env = Environment(loader=file_loader)

    # Create directory
    Path(library_dir_path).mkdir(parents=True, exist_ok=True)

    # Generate various.rs file (global variables used across the library)
    mod_file_path = os.path.join(output_dir_path, library_name + ".rs")
    output_file_path = library_dir_path + "/various.rs"
    with open(output_file_path, 'w') as output_file:
        t = env.get_template("various.rs")
        output_file.write(t.render())
    with open(mod_file_path, 'w') as mod_file:
        mod_file.write("pub mod various;\n")

    # Debug
    print("{} generated.".format(output_file_path))

    # Generate all library files based on arguments
    for json_file_path in json_files:
        # Read JSON data
        data = generate_data_from_json(json_file_path)

        # Get the basename from JSON file name
        basename = os.path.splitext(os.path.basename(json_file_path))[0]

        # Generate library file
        output_file_path = library_dir_path+"/"+basename+".rs"
        with open(output_file_path, 'w') as output_file:
            # TODO: verify that the file exists
            # Generate corresponding template
            t = env.get_template(basename+".rs")
            output_file.write(t.render(data))

            print("{} generated.".format(output_file_path))
            
        with open(mod_file_path, 'a') as mod_file:
            mod_file.write("pub mod "+basename+";\n")
    
    print("")
    
    for extra_file_path in extra_files:
        shutil.copy(extra_file_path, library_dir_path)

        # Get the basename from extra file name
        basename = os.path.splitext(os.path.basename(extra_file_path))[0]

        print("{} generated.".format(os.path.join(library_dir_path,basename)))
            
        with open(mod_file_path, 'a') as mod_file:
            mod_file.write("pub mod "+basename+";\n")

if __name__ == "__main__":
    main()
