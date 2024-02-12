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

    # Create new project
    os.system("cargo new --bin {}".format(args.projectname))

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
    main_orig = "app-template/src/main.rs"
    main_file = args.projectname + "/src/main.rs"
    shutil.copyfile(main_orig, main_file)

    # generate library and add it to src
    os.system("(cd libraryGeneration/libgen && python3 main.py -j ../descriptionFiles/stm32f407/*.json -e ../descriptionFiles/stm32f407/*.rs -o ../../{}/src/stm32rustlib)".format(args.projectname))


    # Cargo.toml
    toml_file = args.projectname + "/Cargo.toml"
    with open(toml_file, 'w') as output_file:
        output_file.write("[package]\n")
        output_file.write("authors = [\"{}\"]\n".format(args.author))
        output_file.write("edition = \"2021\"\n")
        output_file.write("readme = \"README.md\"\n")
        output_file.write("name = \"{}\"\n".format(args.projectname))
        output_file.write("version = \"0.1.0\"\n\n")
        output_file.write("[dependencies]\n")
        output_file.write("geranium_rt = { path = \"../geranium_rt\" }\n")


    # Makefile
    makefile_orig = "app-template/Makefile"
    makefile_file = args.projectname + "/Makefile"
    shutil.copyfile(makefile_orig, makefile_file)

    with open(makefile_file, 'r') as file:
        filedata = file.read()
    filedata = filedata.replace('TARGET =', 'TARGET = {}'.format(args.target))
    filedata = filedata.replace('GDB =', 'GDB = {}'.format(args.gdb))
    filedata = filedata.replace('APP =', 'APP = {}'.format(args.projectname))
    with open(makefile_file, 'w') as file:
        file.write(filedata)


    # Openocd
    openocdcfg_orig = "app-template/openocd.cfg"
    openocdcfg_file = args.projectname + "/openocd.cfg"
    shutil.copyfile(openocdcfg_orig, openocdcfg_file)


    # GDBinit
    gdbinit_orig = "app-template/gdbinit"
    gdbinit_file = args.projectname + "/gdbinit"
    shutil.copyfile(gdbinit_orig, gdbinit_file)


    # README
    readme_orig = "app-template/README.md"
    readme_file = args.projectname + "/README.md"
    shutil.copyfile(readme_orig, readme_file)


if __name__ == "__main__":
    main()
