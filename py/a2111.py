from copy import deepcopy
from prelude import read


class Squids:
    def __init__(self, tbl):
        self.state = deepcopy(tbl)
        self.w, self.h = len(self.state[0]), len(self.state)
        self.cycles = 0

    def adj(self, p):
        """Chebyshev neighborhood of point intersected with valid area."""
        x, y = p
        for y2 in range(max(0, y - 1), min(self.h, y + 2)):
            for x2 in range(max(0, x - 1), min(self.w, x + 2)):
                if x2 == x and y2 == y:
                    continue
                yield (x2, y2)

    def area(self):
        return self.w * self.h

    def points(self):
        return ((x, y) for y in range(self.h) for x in range(self.w))

    def __getitem__(self, key):
        x, y = key
        return self.state[y][x]

    def __setitem__(self, key, value):
        x, y = key
        self.state[y][x] = value

    def cycle(self):
        """Cycle octopi and return number of flashes."""
        self.cycles += 1
        sum = 0

        for p in self.points():
            self[p] += 1
        spent = set()
        resets = 1
        while resets:
            resets = 0
            for p in self.points():
                if self[p] >= 10:
                    resets += 1
                    if p not in spent:
                        spent.add(p)
                        sum += 1
                        for p2 in self.adj(p):
                            if p2 not in spent:
                                self[p2] += 1
                    self[p] = 0

        return sum


if __name__ == "__main__":
    data = read(lambda x: [int(c) for c in x])
    w, h = len(data[0]), len(data)

    # 1
    s = Squids(data)
    print(sum(s.cycle() for _ in range(100)))

    # 2
    s = Squids(data)
    while s.cycle() != s.area():
        pass
    print(s.cycles)
