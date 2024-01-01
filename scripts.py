#!/usr/bin/env python3
import os, subprocess, sys

if not os.environ.get("SCRIPTS_PY_LIBS_BOOTSTRAPPED"):
    sys.exit(subprocess.run([".scripts-py-libs/bootstrap"] + sys.argv).returncode)

import spl.python
from spl import __main__
