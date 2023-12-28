import datetime
import dataclasses
import typing

from dl import djot


def parse_log(dj: djot.Doc):
    return list(map(day_from_section, dj.children))


@dataclasses.dataclass
class Entry:
    start: datetime.time
    end: datetime.time
    tags: typing.Set[typing.List[str]]

def entry_from_pair(pair):
    start, end = pair
    tag_set = set()
    if len(start.children) == 1:
        return
    tags = start.children[1].children[0].children
    for i, tag in enumerate(tags):
        if i % 2 == 1:
            assert isinstance(tag, djot.SoftBreak)
            continue
        tag_set.add(tuple(tag.text.split(" / ")))

    return Entry(
        start=datetime.time.fromisoformat(start.children[0].children[0].text),
        end=datetime.time.fromisoformat(end.children[0].children[0].text),
        tags=tag_set,
    )


@dataclasses.dataclass
class Day:
    date: datetime.date
    entries: typing.List[Entry]


def day_from_section(section: djot.Section):
    return Day(
        date=datetime.date.fromisoformat(section.attributes["id"]),
        entries=list(filter(None, map(entry_from_pair, [(start, end) for start, end in zip(section.children[1:], section.children[2:])]))),
    )
