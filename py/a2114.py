from collections import defaultdict
from prelude import read, cat


def update(table, pairs):
    new_pairs = defaultdict(int)
    for (p, n) in list(pairs.items()):
        c = table[p]
        new_pairs[cat(p[0], c)] += n
        new_pairs[cat(c, p[1])] += n
    return new_pairs


if __name__ == "__main__":
    lines = read()
    init = lines[0]
    table = dict(x.split(" -> ") for x in lines[2:])

    pairs = defaultdict(int)
    for x in (cat(p) for p in zip(init, init[1:])):
        pairs[x] += 1

    # 1, 2
    for n in [10, 40]:
        state = pairs.copy()
        for _ in range(n):
            state = update(table, state)
        hist = defaultdict(int, {init[-1]: 1})
        for ((c, _), n) in state.items():
            hist[c] += n
        print(max(hist.values()) - min(hist.values()))
