import json
import subprocess


def parse_djot_str(s: str):
    p = subprocess.run(["npx", "--yes", "-q", "@djot/djot", "-t", "ast"], check=True, input=s, encoding="utf8", stdout=subprocess.PIPE)
    return json.loads(p.stdout)
