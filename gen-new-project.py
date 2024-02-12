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
    parser.add_argument("-n", "--projectname",
                        help="New project name",
                        required=True)
    parser.add_argument("-t", "--target",
                        help="target board",
                        default="thumbv7em-none-eabi")
    parser.add_argument("-a", "--author",
                        help="project author",
                        default="TBDAuthor")
    parser.add_argument("-gdb", "--gdb",
                        help="New project name",
                        default="gdb")

    args = parser.parse_args()

    # Printing the input for DEBUG.
    print("Given arguments:\n- Output projectname {}\n".format(
            args.projectname
            ))

    return args


def main():

    # Parse argument line
    args = cmdlineParse()

    os.system("cargo new --edition 2018 --bin {}".format(args.projectname))

    # .cargo/config.toml
    config_dest = args.projectname + "/.cargo/config.toml"

    Path(config_dest).parent.mkdir(parents=True, exist_ok=True)

    with open(config_dest, 'w') as output_file:
        output_file.write("[target.{}]\n".format(args.target))
        output_file.write("rustflags = [\"-C\", \"link-arg=-Tmemory.x\"]\n\n")
        output_file.write("[build]\n")
        output_file.write("target = \"{}\"\n".format(args.target))

    # Src folder
    
    # main.rs
    main_orig = "cross-compilation/src/main.rs"
    main_file = args.projectname + "/src/main.rs"
    shutil.copyfile(main_orig, main_file)

    # generate library and add it to src
    os.system("(cd libraryGeneration/libgen && python3 main.py -j ../descriptionFiles/stm32f407/*.json -e ../descriptionFiles/stm32f407/*.rs -o ../../{}/src/stm32rustlib)".format(args.projectname))


    # toml
    toml_file = args.projectname + "/Cargo.toml"

    with open(toml_file, 'w') as output_file:
        output_file.write("[package]\n")
        output_file.write("authors = [\"{}\"]\n".format(args.author))
        output_file.write("edition = \"2018\"\n")
        output_file.write("readme = \"README.md\"\n")
        output_file.write("name = \"{}\"\n".format(args.projectname))
        output_file.write("version = \"0.1.0\"\n\n")
        output_file.write("[dependencies]\n")
        output_file.write("geranium_rt = { path = \"../geranium_rt\" }\n")

    # Makefile
    makefile_orig = "cross-compilation/Makefile"
    makefile_file = args.projectname + "/Makefile"
    shutil.copyfile(makefile_orig, makefile_file)

    # Read in the file
    with open(makefile_file, 'r') as file:
        filedata = file.read()

    # Replace the target string
    filedata = filedata.replace('TARGET =', 'TARGET = {}'.format(args.target))
    filedata = filedata.replace('GDB =', 'GDB = {}'.format(args.gdb))
    filedata = filedata.replace('APP =', 'APP = {}'.format(args.projectname))

    # Write the file out again
    with open(makefile_file, 'w') as file:
        file.write(filedata)

    # Openocd
    openocdcfg_orig = "cross-compilation/openocd.cfg"
    openocdcfg_file = args.projectname + "/openocd.cfg"
    shutil.copyfile(openocdcfg_orig, openocdcfg_file)

    # GDBinit
    gdbinit_orig = "cross-compilation/gdbinit"
    gdbinit_file = args.projectname + "/gdbinit"
    shutil.copyfile(gdbinit_orig, gdbinit_file)
    
    # README
    readme_orig = "cross-compilation/README.md"
    readme_file = args.projectname + "/README.md"
    shutil.copyfile(readme_orig, readme_file)
        
'''
TARGET = thumbv7m-none-eabi
TARGET = thumbv7em-none-eabi
OPENOCD_CFG = openocd.cfg 
GDB = gdb
GDB = arm-none-eabi-gdb

openocd :
	openocd -f $(OPENOCD_CFG) 	

debug :
	cargo clean
	cargo build

objdump :
	cargo clean
	cargo build
	arm-none-eabi-objdump -h target/$(TARGET)/debug/app

flashdebug :
	cargo clean
	cargo build
	$(GDB) -q target/$(TARGET)/debug/app -ex "target remote :3333" -ex "load"

flashrelease :
	cargo clean
	cargo build --release 
	$(GDB) -q target/$(TARGET)/release/app -ex "target remote :3333" -ex "load"

'''

        

    # Create directory
    #Path(args.projectname).mkdir(parents=True, exist_ok=True)


if __name__ == "__main__":
    main()
