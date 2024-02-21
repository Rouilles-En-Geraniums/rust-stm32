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
    parser.add_argument("-f","--force",
                        help="Specify whether or not to force update the project folder if it already exists",
                        action=argparse.BooleanOptionalAction)
    parser.add_argument("-t", "--target",
                        help="target board",
                        default="thumbv7em-none-eabi")
    parser.add_argument("-a", "--author",
                        help="project author",
                        default="TBDAuthor")
    parser.add_argument("-g", "--gdb",
                        help="Specify the GDB command to use",
                        default="gdb")
    parser.add_argument("-p", "--openocdcfg",
                        help="Specify the path of the openOCD config file to use",
                        default="app-template/openocd.cfg")
    parser.add_argument("-u","--updatelibrary",
                        help="Specify whether or not to update the library crate. This option disables every other options",
                        action=argparse.BooleanOptionalAction)
    parser.add_argument ("-s", "--scheduler",
                        help="Generates a Scheduled project, with tasks defined in user_tasks.rs scheduled according to a given user-defined scheduling. More info on ./taskSequencing/README.md",
                        action=argparse.BooleanOptionalAction)

    args = parser.parse_args()

    return args


def main():

    # Parse argument line
    args = cmdlineParse()

    projectname = args.projectname
    force = args.force
    target = args.target
    author = args.author
    gdb = args.gdb
    openocdcfg = args.openocdcfg
    updatelibrary = args.updatelibrary

    # Update geranium_rt library
    if (updatelibrary):
        os.system("(cd libraryGeneration/libgen && python3 main.py -j ../descriptionFiles/stm32f407/*.json -e ../descriptionFiles/stm32f407/*.rs -o ../../geranium_rt/src -l stm32rustlib)")

    else:

        # Create new project
        if not os.path.isdir(projectname):
            os.system("cargo new --bin {}".format(projectname))
        else:
            if (force):
                print("Info: folder {} already exists. If the newly generated project does not compile properly, try deleting the folder first, or chose a different project name.".format(projectname))
            else:
                print("Info: folder {} already exists. If the newly generated project does not compile properly, try deleting the folder first, or chose a different project name.\nTo overwrite this error and replace the files currently in this folder, add the -f option.".format(projectname))
                sys.exit(1)

        # .cargo/config.toml
        config_dest = projectname + "/.cargo/config.toml"

        Path(config_dest).parent.mkdir(parents=True, exist_ok=True)

        with open(config_dest, 'w') as output_file:
            output_file.write("[target.{}]\n".format(target))
            output_file.write("rustflags = [\"-C\", \"link-arg=-Tmemory.x\"]\n\n")
            output_file.write("[build]\n")
            output_file.write("target = \"{}\"\n".format(target))


        # Src folder
        # main.rs and user_tasks.rs

        if args.scheduler:
            user_tasks_orig = "app-template/sequencer/user_tasks.rs"
            user_tasks_file = projectname + "/src/user_tasks.rs"
            shutil.copyfile(user_tasks_orig, user_tasks_file)
            main_orig = "app-template/sequencer/main_scheduled_project.rs"
            main_file = projectname + "/src/main.rs"
            shutil.copyfile(main_orig, main_file)
        else:
            main_orig = "app-template/src/main.rs"
            main_file = projectname + "/src/main.rs"
            shutil.copyfile(main_orig, main_file)

        # Cargo.toml
        toml_file = projectname + "/Cargo.toml"
        with open(toml_file, 'w') as output_file:
            output_file.write("[package]\n")
            output_file.write("authors = [\"{}\"]\n".format(author))
            output_file.write("edition = \"2021\"\n")
            output_file.write("name = \"{}\"\n".format(projectname))
            output_file.write("version = \"0.1.0\"\n\n")
            output_file.write("[dependencies]\n")
            output_file.write("geranium_rt = { path = \"../geranium_rt\" }\n")
            if args.scheduler:
                output_file.write("geranium_seq = { path = \"../geranium_seq\" }\n")


        # Makefile
        makefile_orig = "app-template/Makefile"
        makefile_file = projectname + "/Makefile"
        shutil.copyfile(makefile_orig, makefile_file)

        with open(makefile_file, 'r') as file:
            filedata = file.read()
        filedata = filedata.replace('TARGET =', 'TARGET = {}'.format(target))
        filedata = filedata.replace('GDB =', 'GDB = {}'.format(gdb))
        filedata = filedata.replace('APP =', 'APP = {}'.format(projectname))
        with open(makefile_file, 'w') as file:
            file.write(filedata)

        # Scheduler Generator
        if args.scheduler:
            generator_origin = "taskSequencing/codeGen/main.py"
            generator_file = projectname + "/generate-main.py"
            shutil.copyfile(generator_origin, generator_file)
            schedule_origin = "taskSequencing/codeGen/schedule.json"
            schedule_file = projectname + "/schedule.json"
            shutil.copyfile(schedule_origin, schedule_file)

        # Openocd
        openocdcfg_orig = openocdcfg
        openocdcfg_file = projectname + "/openocd.cfg"
        shutil.copyfile(openocdcfg_orig, openocdcfg_file)


        # GDBinit
        gdbinit_orig = "app-template/gdbinit"
        gdbinit_file = projectname + "/gdbinit"
        shutil.copyfile(gdbinit_orig, gdbinit_file)


        print("\nProject '{}' has been successfully generated.".format(projectname))


if __name__ == "__main__":
    main()
