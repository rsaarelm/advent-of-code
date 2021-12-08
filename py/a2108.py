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
    def comp(s):
        return set("abcdefg") - s

    # One.
    (cf,) = [set(x) for x in wires if len(x) == 2]
    # Seven.
    (acf,) = [set(x) for x in wires if len(x) == 3]
    # Four.
    (bcdf,) = [set(x) for x in wires if len(x) == 4]
    # Shared in 2, 3, 5.
    adg = set.intersection(*[set(x) for x in wires if len(x) == 5])
    # Shared in 0, 6, 9.
    abfg = set.intersection(*[set(x) for x in wires if len(x) == 6])
    # We already know 8 is abcdefg is abcdefg.

    # Solve manually.
    (a,) = acf - cf
    bd = bcdf - cf
    (b,) = bd - adg
    (d,) = bd - {b}
    bcef = comp(adg)
    (e,) = bcef - {b} - cf
    cde = comp(abfg)
    (f,) = cf - cde
    (c,) = cf - {f}
    (g,) = comp({a, b, c, d, e, f})

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
