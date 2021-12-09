from functools import reduce
from prelude import read
from typing import List


def get(map, p):
    x, y = p
    if x < 0 or y < 0 or x >= len(map[0]) or y >= len(map):
        return None
    return map[y][x]


def neighbors(map, p):
    x, y = p
    return [
        p
        for p in [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
        if get(map, p) is not None
    ]


def flood(map, p) -> set:
    result = set()
    open = {p}
    while open:
        seed = open.pop()
        result.add(seed)
        for n in neighbors(map, seed):
            cell = get(map, n)
            if n not in result and cell is not None and cell < 9:
                open.add(n)
    return result


if __name__ == "__main__":
    map = read(lambda y: [int(x) for x in y])

    # 1
    acc = 0
    for y in range(len(map)):
        for x in range(len(map[0])):
            cell = get(map, (x, y))
            if all(cell < get(map, n) for n in neighbors(map, (x, y))):
                acc += cell + 1
    print(acc)

    # 2
    basins: List[set] = []
    for y in range(len(map)):
        for x in range(len(map[0])):
            cell = get(map, (x, y))
            if cell == 9:
                continue
            if any((x, y) in b for b in basins):
                continue
            basins.append(flood(map, (x, y)))
    print(
        reduce(
            lambda x, y: x * y,
            list(sorted((len(b) for b in basins), reverse=True))[:3],
        )
    )
