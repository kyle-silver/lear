"""
borrowed with love from http://shakespeare.mit.edu/lear/full.html
"""
from __future__ import annotations

import json
from dataclasses import dataclass
from os import path
from typing import List

from bs4 import BeautifulSoup
from bs4.element import Tag
from dataclasses_json import DataClassJsonMixin
from more_itertools import peekable


@dataclass(frozen=True)
class Line(DataClassJsonMixin):
    """A bit of text from the play"""


@dataclass(frozen=True)
class SpokenLine(Line):
    """Spoken text"""

    text: str


@dataclass(frozen=True)
class StageDirection(Line):
    """A stage direction, which will be formatted differently"""

    direction: str


@dataclass(frozen=True)
class TextBlock(DataClassJsonMixin):
    """Base element for display"""


@dataclass(frozen=True)
class Dialogue(TextBlock):
    """Represents a block of dialogue spoken by a single character"""

    character: str
    act: int
    scene: int
    start: int
    stop: int
    lines: List[Line]

    @staticmethod
    def from_html(a: Tag, blockquote: Tag) -> Dialogue:
        """Parses out the weird MIT formatting"""
        character = a.text
        lines = list(blockquote)
        dialogue_lines = [line for line in lines if line.name == "a"]
        # we need to grab some numbers for the citation (act.scene.start-stop)
        (first_line, last_line) = (dialogue_lines[0], dialogue_lines[-1])
        [act, scene, start] = [int(token) for token in first_line["name"].split(".")]
        [_, _, end] = [int(token) for token in last_line["name"].split(".")]
        # now we need to categorize all of the lines as either stage directions
        # or actual dialogue
        contents: List[Line] = []
        for line in lines:
            if line.name == "a":
                contents.append(SpokenLine(line.text))
            elif line.name == "p":
                contents.append(StageDirection(line.text))
        return Dialogue(character, act, scene, start, end, contents)


@dataclass(frozen=True)
class SceneInfo(TextBlock):
    act: int
    setting: str
    staging: StageDirection

    @staticmethod
    def from_html(act: int, setting: Tag, staging: Tag) -> SceneInfo:
        return SceneInfo(act, setting.text.strip(), staging.text.strip())


# read file
working_dir = path.dirname(__file__)
with open(path.join(working_dir, "lear.html"), "r", encoding="utf-8") as f:
    lear_html = f.read()

# get the tags we care about
soup = BeautifulSoup(lear_html, "html.parser")
lear = peekable(tag for tag in soup.body if tag.name is not None)

scenes: List[List[TextBlock]] = []
current_scene: List[TextBlock] = []
act = 0

while lear.peek(None):
    tag: Tag = next(lear)
    # Scene indicators, we want to skip these
    if tag.name == "h3":
        scenes.append(current_scene)
        if tag.text.startswith("ACT"):
            act = act + 1
            tag = next(lear)
        staging = next(lear)
        scene_info = SceneInfo.from_html(act, tag, staging)
        current_scene.append(scene_info)
        continue
    # characters giving speeches
    blockquote: Tag = next(lear)
    dialogue = Dialogue.from_html(tag, blockquote)
    current_scene.append(dialogue)

for (index, scene) in enumerate(scenes):
    print(f"{index:02d}.json")
    file = path.join(working_dir, "scenes", f"{index:02d}.json")
    with open(file, "w", encoding="utf-8") as f:
        data = [block.to_dict() for block in scene]
        json.dump(data, f)
