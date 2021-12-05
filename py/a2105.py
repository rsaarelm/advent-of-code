from collections import defaultdict
from prelude import read

def orthogonal(line):
    (x1, y1), (x2, y2) = line
    return x1 == x2 or y1 == y2

def points(line):
    # XXX: Only works for orthogonal or diagonal lines
    (x1, y1), (x2, y2) = line
    dx = x2 - x1
    dy = y2 - y1
    length = max(abs(dx), abs(dy))
    dx, dy = dx / length, dy / length

    x, y = x1, y1
    for _ in range(length + 1):
        yield (x, y)
        x += dx
        y += dy

if __name__ == "__main__":
    data = read(
        lambda line: [
            [int(x) for x in y.split(",")] for y in line.split("->")
        ]
    )

    # 1
    lines = [line for line in data if orthogonal(line)]
    hist = defaultdict(int)

    for line in lines:
        for point in points(line):
            hist[point] += 1

    print(len([point for point in hist if hist[point] > 1]))

    # 2
    lines = data[:]
    hist = defaultdict(int)

    for line in lines:
        for point in points(line):
            hist[point] += 1

    print(len([point for point in hist if hist[point] > 1]))
