from prelude import read


def parse_wire(line):
    for elt in line.split(","):
        if elt[0] == "U":
            yield [0, -int(elt[1:])]
        if elt[0] == "R":
            yield [int(elt[1:]), 0]
        if elt[0] == "D":
            yield [0, int(elt[1:])]
        if elt[0] == "L":
            yield [-int(elt[1:]), 0]


if __name__ == "__main__":
    data = read(lambda a: list(parse_wire(a)))
    print(data)
