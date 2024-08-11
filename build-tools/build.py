import argparse
import kdl
from pprint import pprint
from typing import List
from tools.types import build_types
from tools.dict import build_dicts
from tools.record import record_tapes

def parse_config(file_path):
    with open(file_path, "r") as docf:
        doc = kdl.parse(docf.read())
        return {
            "target": {
                "tapes": doc["target"]["tapes"].args[0],
                "dicts": doc["target"]["dicts"].args[0],
                "schemas": doc["target"]["schemas"].args[0],
                "binaries": doc["target"]["binaries"].args[0],
                "redb": doc["target"]["log_redb"].args[0]
            },
            "types": {
                child.name: {
                    "checkpoint": int(child.props.get("checkpoint")) if child.props.get("checkpoint") else None,
                    "use_dict": child.props.get("use-dict", False),
                }
                for child in doc["entities"].nodes
            },
        }


parser = argparse.ArgumentParser(prog="vhs-build")
parser.add_argument("-c", "--conf", type=parse_config, required=True)
parser.add_argument("-a", "--all", action="store_true")
parser.add_argument("-t", "--types", action="append")

subparsers = parser.add_subparsers(required=True)

type_tool_args = subparsers.add_parser("schema")
type_tool_args.set_defaults(func=build_types)

dict_tool_args = subparsers.add_parser("dict")
dict_tool_args.set_defaults(func=build_dicts)

tape_recorder_args = subparsers.add_parser("record")
tape_recorder_args.set_defaults(func=record_tapes)

args = parser.parse_args()
if not args.types:
    vars(args)["all"] = True

if args.all:
    vars(args)["types"] = args.conf["types"]
else:
    vars(args)["types"] = {
        k: v for (k, v) in args.conf["types"].items() if k in args.types
    }

args.func(args)
