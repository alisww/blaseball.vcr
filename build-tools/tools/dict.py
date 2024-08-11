import subprocess
import os.path
from colored import fg, attr
from pprint import pprint

def build_dicts(args):
    binary = os.path.join(args.conf["target"]["binaries"], "train_vhs_dict")
    
    types = args.types

    if args.all:
        types = {k:v for (k,v) in args.types.items() if v['use_dict']}

    for i, (type_name, conf) in enumerate(types.items()):
        print(
            f"{attr(1)}-// building dict for {type_name}! ({i+1}/{len(types)}) -//{attr(0)}"
        )
        out_path = os.path.join(args.conf["target"]["dicts"], f"{type_name}.dict")

        cmd = [binary]

        if conf['checkpoint']:
            cmd.extend(["-c", str(conf['checkpoint'])])

        cmd.extend(["-o", out_path, type_name])
        print(cmd)
        subprocess.run(cmd)
