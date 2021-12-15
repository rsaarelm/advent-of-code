from heapq import heappush, heappop
from prelude import read, neighbors4


def astar(start, end, cost):
    def h(p):
        return abs(end[0] - p[0]) + abs(end[1] - p[1])

    g = {(0, 0): 0}
    open = []
    come_from = {}
    heappush(open, (h(start) + g[start], start))

    while open:
        # Best current candidate.
        _, p = heappop(open)

        if p == end:
            break

        for p2 in neighbors4(p):
            p2_cost = g[p] + cost(p2)
            if p2 in g and g[p2] <= p2_cost:
                continue

            g[p2] = g[p] + cost(p2)
            come_from[p2] = p
            heappush(open, ((h(p2) + g[p2], p2)))

    # Crash if no path was found.
    assert(end in come_from)

    # Reconstruct path
    path = [end]
    while come_from[path[-1]] != start:
        path.append(come_from[path[-1]])
    return sum(cost(p) for p in path)


if __name__ == "__main__":
    data = read(lambda x: [int(c) for c in x])
    w, h = len(data[0]), len(data)

    def cost(p):
        x, y = p
        if x < 0 or y < 0 or x >= w or y >= h:
            return 999999999
        else:
            return data[y][x]
    print(astar((0, 0), (w - 1, h - 1), cost))

    def cost2(p):
        x, y = p
        if x < 0 or y < 0 or x >= w * 5 or y >= h * 5:
            return 999999999
        u, v = (x % w, y % h)
        n = data[v][u]
        inc = x // w + y // h
        n = ((n - 1 + inc) % 9) + 1
        return n
    print(astar((0, 0), (w*5 - 1, h*5 - 1), cost2))
