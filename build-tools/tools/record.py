import subprocess
import os.path
from colored import fg, attr

def record_tapes(args):
    binary = os.path.join(args.conf["target"]["binaries"], "encode_vhs")
    for i, (type_name, conf) in enumerate(args.types.items()):
        print(
            f"{attr(1)}-// recording tape for {type_name}! ({i+1}/{len(args.types)}) -//{attr(0)}"
        )
        
        out_path = os.path.join(args.conf["target"]["tapes"], f"{type_name}.vhs")

        cmd = [binary]

        if conf['checkpoint']:
            cmd.extend(["-c", str(conf['checkpoint'])])

        if conf['use_dict']:
            dict_path = os.path.join(args.conf["target"]["dicts"], f"{type_name}.dict")
            cmd.extend(["-d", dict_path])

        cmd.extend(["--table", args.conf["target"]["redb"]])
        cmd.extend(["-o", out_path, type_name])
        

        subprocess.run(cmd)
