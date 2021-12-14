from prelude import read, cat, histogram, eprint

def expand(table, line):
    ret = []
    for x in (cat(p) for p in zip(line, line[1:])):
        ret.append(x[0])
        try:
            ret.append(table[x])
        except KeyError:
            pass
    ret.append(line[-1])
    return cat(ret)


if __name__ == '__main__':
    lines = read()
    init = lines[0]
    table = dict(x.split(' -> ') for x in lines[2:])

    # 1
    x = init
    for _ in range(10):
        x = expand(table, x)

    hist = histogram(x)
    print(max(hist.values()) - min(hist.values()))
