from copy import deepcopy
from prelude import read

if __name__ == '__main__':
    data = read(lambda x: [int(c) for c in x])
    w, h = len(data[0]), len(data)

    def adj(x, y):
        """Chebyshev neighborhood of (x, y) intersected with valid area."""
        for i in [-1, 0, 1]:
            for j in [-1, 0, 1]:
                xx, yy = x + i, y + j
                if (i or j) and xx >= 0 and yy >= 0 and xx < w and yy < h:
                    yield (xx, yy)

    state = deepcopy(data)
    a = 0
    for _ in range(100):
        for y in range(h):
            for x in range(w):
                state[y][x] += 1
        spent = set()
        while True:
            cycles = 0
            for y in range(h):
                for x in range(w):
                    if state[y][x] >= 10:
                        cycles += 1
                        if (x, y) not in spent:
                            spent.add((x, y))
                            a += 1
                            for (x2, y2) in adj(x, y):
                                if not (x2, y2) in spent:
                                    state[y2][x2] += 1
                        state[y][x] = 0
            if not cycles:
                break

    print(a)

    state = deepcopy(data)
    n = 0
    while True:
        a = 0
        for y in range(h):
            for x in range(w):
                state[y][x] += 1
        spent = set()
        while True:
            cycles = 0
            for y in range(h):
                for x in range(w):
                    if state[y][x] >= 10:
                        cycles += 1
                        if (x, y) not in spent:
                            spent.add((x, y))
                            a += 1
                            for (x2, y2) in adj(x, y):
                                if not (x2, y2) in spent:
                                    state[y2][x2] += 1
                        state[y][x] = 0
            if not cycles:
                break
        n += 1
        if a == w * h:
            break
    print(n)
