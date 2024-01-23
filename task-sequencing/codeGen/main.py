import sys
import os
import json
from pathlib import Path
import argparse

"""
    Error codes:
        1 : Json file not found
        [50-60[: Json error
            50: Json parsing error
            51: mandatory key is absent in json
            52: key present but value is unexpected
"""
FILE_NOT_FOUND = 1
JSON_PARSING_ERROR = 50
MANDATORY_KEY_ABSENT = 51
UNEXPECTED_VALUE = 52

DEBUG = False


def cmdlineParse():
    # Argument line parser initialisation.
    parser = argparse.ArgumentParser(
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )

    # Parser argument control section.
    parser.add_argument("json_file", help="json file containing ordonnancing")

    args = parser.parse_args()

    # Printing the input for DEBUG.
    if DEBUG:
        print("Given arguments:\n- JSON {args.json_file}\n")

    return args


def check_json_validity(json_data):
    # TODO Versionning see https://stackoverflow.com/questions/11887762/how-do-i-compare-version-numbers-in-python
    TOP_LEVEL_KEYS = ["version", "type", "hyperperiod", "tasks", "jobs"]
    VALID_TYPES = ["strict", "deadline", "cooperative", "preemptive"]
    TASKS_KEYS = ["id", "functionName"]  # "label" key is optionnal
    JOBS_KEYS = ["taskId", "duration", "startTime"]

    # check top level keys presence
    for key in TOP_LEVEL_KEYS:
        if key not in json_data:
            print(f"[ERROR] '{key}' not in json", file=sys.stderr)
            sys.exit(MANDATORY_KEY_ABSENT)

    if json_data["type"] not in VALID_TYPES:
        print(f"[ERROR] '{json['type']}' is not in valid type list : '{VALID_TYPES}'", file=sys.stderr)
        sys.exit(UNEXPECTED_VALUE)

    # check tasks keys presence
    tasks = json_data["tasks"]
    for key in TASKS_KEYS:
        for task in tasks:
            if key not in task:
                print(f"[ERROR] '{key}' not in task : '{task}'", file=sys.stderr)
                sys.exit(MANDATORY_KEY_ABSENT)

    task_ids = [task["id"] for task in tasks]

    # check jobs (keys)
    jobs = json_data["jobs"]
    for key in JOBS_KEYS:
        for job in jobs:
            if key not in job:
                print(f"[ERROR] '{key}' not in job : '{job}'", file=sys.stderr)
                sys.exit(MANDATORY_KEY_ABSENT)

    # check if taskId reference a previously declared task id's
    for job in jobs:
        if job["taskId"] not in task_ids:
            print(f"[ERROR] job: '{job}' taskId field's : '{job['taskId']}' "
                  f"doesn't reference a previously declared task id's : {task_ids}")
            sys.exit(UNEXPECTED_VALUE)


def parse_json(json_file_path):
    try:
        json_file = open(json_file_path, 'r')
        ordo = json.load(json_file)
    except FileNotFoundError as e:
        print(f"[ERROR] File '{json_file_path}' not found.", file=sys.stderr)
        print(f"{e}", file=sys.stderr)
        sys.exit(FILE_NOT_FOUND)
    except json.JSONDecodeError as e:
        print(f"[ERROR] Unable to parse JSON in file '{json_file_path}'.", file=sys.stderr)
        print(f"{e}", file=sys.stderr)
        sys.exit(JSON_PARSING_ERROR)

    check_json_validity(ordo)
    tasks = {task["id"]: task for task in ordo["tasks"]}

    json_file.close()


def main():
    # Parse argument line
    args = cmdlineParse()
    json_file = args.json_file

    # Parse JSON (check errors)
    parse_json(json_file)


if __name__ == "__main__":
    main()
