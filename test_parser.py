import datetime
import pathlib

from dl import djot, log


def test_raw_parse():
    djot.raw_parse(pathlib.Path("example.djot").read_text())


def test_parse():
    djot.struct_to_classes(djot.raw_parse(pathlib.Path("example.djot").read_text()))


def test_parse_log():
    l = log.parse_log(djot.struct_to_classes(djot.raw_parse(pathlib.Path("example.djot").read_text())))
    assert l == [
        log.Day(date=datetime.date(2023, 12, 3), entries=[
            log.Entry(start=datetime.time(9, 0), end=datetime.time(13, 0), tags={('Work', 'MyOrg', 'MyDept', 'MyProj'), ('Coding',)}),
            log.Entry(start=datetime.time(14, 0), end=datetime.time(15, 0), tags={('Meeting',), ('Work', 'MyOrg', 'MyDept')}),
            log.Entry(start=datetime.time(15, 0), end=datetime.time(18, 0), tags={('Work', 'MyOrg', 'MyDept', 'MyProj'), ('Coding',)})]),
        log.Day(date=datetime.date(2023, 12, 4), entries=[
            log.Entry(start=datetime.time(9, 0), end=datetime.time(13, 0), tags={('Work', 'MyOrg', 'MyDept', 'MyProj'), ('Coding',)}),
            log.Entry(start=datetime.time(14, 0), end=datetime.time(18, 0), tags={('Work', 'MyOrg', 'MyDept', 'MyProj'), ('Coding',)})])]
