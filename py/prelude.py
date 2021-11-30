import re
import sys
from typing import IO, Tuple


def read(parse=str, sep: str = '\n', f: IO = sys.stdin):
    """Read homogeneous `sep` separated input from given file."""
    return [parse(elt) for elt in f.read().rstrip().split(sep)]


def ints(text: str) -> Tuple[int, ...]:
    """Return tuple of integers extracted from arbitrary text."""
    return tuple(map(int, re.findall('-?[0-9]+', text)))
