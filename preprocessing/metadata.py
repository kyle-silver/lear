import json
from os import path
from typing import Any, Dict, List

working_dir = path.dirname(__file__)

for index in range(1, 27):
    with open(
        path.join(working_dir, "scenes", f"{index:02d}.json"), "r", encoding="utf-8"
    ) as f:
        data: List[Dict[str, Any]] = json.load(f)
        lines = data[-1]["Dialogue"]["end"]
        print(f"    Metadata {{ act: 1, scene: 1, lines: {lines} }},")
