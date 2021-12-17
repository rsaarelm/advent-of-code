from math import copysign
from prelude import read, ints


def plot(dx, dy, rect):
    x1, x2, y1, y2 = rect
    x, y = 0, 0
    while x <= x2 and y >= y1:
        x += dx
        y += dy
        if dx:
            dx -= int(copysign(1, dx))
        dy -= 1
        yield x, y


def hits(dx, dy, rect):
    x1, x2, y1, y2 = rect
    for (x, y) in plot(dx, dy, rect):
        if x >= x1 and x <= x2 and y >= y1 and y <= y2:
            return True
        if x > x2 or y < y1:
            return False


def xsum(dx):
    return (dx * dx + dx) / 2


if __name__ == "__main__":
    rect = ints(read()[0])
    x1, x2, y1, y2 = rect

    # Find all dx values that stop on top of the box.
    dxes = [dx for dx in range(9999) if xsum(dx) >= x1 and xsum(dx) <= x2]

    max_y = 0
    for dx in dxes:
        for dy in (dy for dy in range(999) if hits(dx, dy, rect)):
            apex = max(y for (x, y) in plot(dx, dy, rect))
            max_y = max(max_y, apex)

    print(max_y)

    print(
        len(
            [
                (dx, dy)
                for dx in range(5000)
                for dy in range(y1, 5000)
                if hits(dx, dy, rect)
            ]
        )
    )
