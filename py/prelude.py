import re
import sys
from typing import IO, Tuple


def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def input_str(*, _ret=[]):
    """Get input from stdin.

    Can be called repeatedly and will return same value."""
    if not _ret:
        _ret.append(sys.stdin.read().rstrip())
    return _ret[0]


def read(parse=str, sep: str = "\n", f: IO = sys.stdin):
    """Read homogeneous `sep` separated input from given file."""
    return [parse(elt) for elt in input_str().split(sep)]


def ints(text: str) -> Tuple[int, ...]:
    """Return tuple of integers extracted from arbitrary text."""
    return tuple(map(int, re.findall("-?[0-9]+", text)))


def re_match(r: str, text: str):
    matches = list(re.search(r, text).groups())
    for i in range(len(matches)):
        # Opportunistically convert to integers
        try:
            matches[i] = int(matches[i])
        except ValueError:
            pass
    return tuple(matches)
