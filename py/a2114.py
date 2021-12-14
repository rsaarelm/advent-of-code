from collections import defaultdict
from prelude import read, cat


def update(table, pairs):
    new_pairs = defaultdict(int)
    for (p, n) in list(pairs.items()):
        c = table[p]
        new_pairs[cat(p[0], c)] += n
        new_pairs[cat(c, p[1])] += n
    return new_pairs


def pairs_hist(init, pairs):
    hist = defaultdict(int)
    for p in pairs:
        hist[p[0]] += pairs[p]
    # The only char not accounted for by first elements of pairs.
    hist[init[-1]] += 1
    return hist


if __name__ == "__main__":
    lines = read()
    init = lines[0]
    table = dict(x.split(" -> ") for x in lines[2:])

    pairs = defaultdict(int)
    for x in (cat(p) for p in zip(init, init[1:])):
        pairs[x] += 1

    # 1
    state = pairs.copy()
    for _ in range(10):
        state = update(table, state)
        pairs_hist(init, state)
    hist = pairs_hist(init, state)
    print(max(hist.values()) - min(hist.values()))

    # 2
    state = pairs.copy()
    for _ in range(40):
        state = update(table, state)
        pairs_hist(init, state)
    hist = pairs_hist(init, state)
    print(max(hist.values()) - min(hist.values()))
