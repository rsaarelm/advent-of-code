from prelude import neighbors8, read


class Grid:
    def __init__(self, input_lines):
        self.codebook = [c == "#" for c in input_lines[0]]
        # Enhancement generation.
        self.n = 0
        # Elements inside the grid
        self.grid = {
            (x, y): c == "#"
            for y, line in enumerate(input_lines[2:])
            for x, c in enumerate(line)
        }

    @property
    def unmapped(self):
        """Value of cells outside the map."""
        if self.codebook[0]:
            return self.n % 2 != 0
        else:
            return False

    def points(self):
        """All points that might have content in their 3x3 neighborhood."""
        ret = set()
        for p in self.grid:
            for u in neighbors8(p):
                ret.add(u)
        return ret

    def point_next(self, p):
        ret = 0
        for (i, q) in enumerate(reversed(list(neighbors8(p)))):
            if self[q]:
                ret += 1 << i
        return self.codebook[ret]

    def tick(self):
        self.grid = {p: self.point_next(p) for p in self.points()}
        self.n += 1

    def len(self):
        assert not self.unmapped, "infinity"
        return len([1 for p in self.grid if self[p]])

    def __getitem__(self, p):
        return self.grid.get(p, self.unmapped)

    def __str__(self):
        ps = self.points()
        xs = {x for (x, _) in ps}
        ys = {y for (_, y) in ps}
        ret = ""
        for y in range(min(ys) - 1, max(ys) + 2):
            for x in range(min(xs) - 1, max(xs) + 2):
                if self[(x, y)]:
                    ret += "██"
                else:
                    ret += " ."
            ret += "\n"
        return ret


if __name__ == "__main__":
    grid = Grid(read())

    # 1
    for _ in range(2):
        grid.tick()
    print(grid.len())

    # 2
    for _ in range(48):
        grid.tick()
    print(grid.len())
