import dataclasses
import json
import subprocess
import typing


def raw_parse(s: str):
    p = subprocess.run(["npx", "--yes", "-q", "@djot/djot", "-t", "ast"], check=True, input=s, encoding="utf8", stdout=subprocess.PIPE)
    return json.loads(p.stdout)


def struct_to_classes(s):
    assert s["tag"] in _TAG_TO_TYPE, s
    return _TAG_TO_TYPE[s["tag"]](**s)


@dataclasses.dataclass
class Block:
    pass


@dataclasses.dataclass
class Inline:
    pass


@dataclasses.dataclass
class HasAttributes:
    attributes: typing.Any



@dataclasses.dataclass
class Doc:
    tag: str
    references: typing.Any
    footnotes: typing.Any
    children: typing.List[Block]

    def __init__(self, tag, references, footnotes, children):
        self.tag = tag
        self.references = references
        self.footnotes = footnotes
        self.children = [struct_to_classes(i) for i in children]


@dataclasses.dataclass
class Section(Block, HasAttributes):
    tag: str
    children: typing.List[Block]

    def __init__(self, tag, children, attributes):
        self.tag = tag
        self.children = [struct_to_classes(i) for i in children]
        self.attributes = attributes


@dataclasses.dataclass
class Heading(Block):
    tag: str
    level: int
    children: typing.List[Inline]

    def __init__(self, tag, level, children):
        self.tag = tag
        self.children = [struct_to_classes(i) for i in children]
        self.level = level


@dataclasses.dataclass
class String(Inline):
    tag: str
    text: str


@dataclasses.dataclass
class SoftBreak(Inline):
    tag: str


_TAG_TO_TYPE = {
    "doc": Doc,
    "section": Section,
    "heading": Heading,
    "str": String,
    "soft_break": SoftBreak,
    "bullet_list": dict,
}
