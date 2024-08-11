import requests
import json

seasons = []

with open("seasons.list.txt") as f:
    for line in f:
        line = line.split(",")
        sim = line[0][1:-1]
        season = line[1].strip()
        r = requests.get(f"https://api.sibr.dev/eventually/v2/time/{sim}/{season}").json()
        seasons.append({
            "sim": sim,
            "season": season,
            "start": r["start"],
            "end": r["end"]
        })

with open("seasons.json", "w") as outf:
    json.dump(seasons, outf)