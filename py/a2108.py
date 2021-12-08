from prelude import read
from typing import Iterable, Set

#   aaaa
#  b    c
#  b    c
#   dddd
#  e    f
#  e    f
#   gggg

DIGITS: dict[str, str] = {
    "cf": "1",
    # ---
    "acf": "7",
    # ---
    "bcdf": "4",
    # ---
    "acdeg": "2",
    "acdfg": "3",
    "abdfg": "5",
    # ---
    "abcefg": "0",
    "abdefg": "6",
    "abcdfg": "9",
    # ---
    "abcdefg": "8",
}


def solve(wires: Iterable[str]) -> dict[str, str]:
    # One
    (cf,) = [set(x) for x in wires if len(x) == 2]
    # Seven
    (acf,) = [set(x) for x in wires if len(x) == 3]
    # Four
    (bcdf,) = [set(x) for x in wires if len(x) == 4]
    # Full set
    abcdefg = set("abcdefg")

    # Shared segments in 2, 3, 5:
    adg: Set[str] = set.intersection(*[set(x) for x in wires if len(x) == 5])
    bcef = abcdefg - adg

    # Shared segments in 0, 6, 9
    abfg: Set[str] = set.intersection(*[set(x) for x in wires if len(x) == 6])
    dce = abcdefg - abfg

    # Manually derived simplification:
    (a,) = acf - cf
    bd = bcdf - cf
    (b,) = bd - adg
    cf = bcdf - bd
    (g,) = abfg - {a} - bd - cf
    (d,) = adg - {a} - {g}
    (f,) = cf - dce
    (e,) = bcef - {b} - cf
    (c,) = cf - {f}

    return {a: "a", b: "b", c: "c", d: "d", e: "e", f: "f", g: "g"}


if __name__ == "__main__":
    data = read(lambda y: [x.split(" ") for x in y.split(" | ")])

    # One is two, seven is three, four is four, eight is seven.
    print(
        sum(
            len(segments) in [2, 3, 4, 7]
            for (_, num) in data
            for segments in num
        )
    )

    sum = 0
    for (wires, digits) in data:
        sol = solve(wires)
        num = int(
            "".join(
                DIGITS["".join(sorted(sol[c] for c in digit))]
                for digit in digits
            )
        )
        sum += num
    print(sum)
