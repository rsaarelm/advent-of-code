from fractions import Fraction
from functools import reduce
from prelude import read, eprint


def step_reduce(expr):
    def flatten(expr, depth=0):
        """Flatten expr into a list with depth values."""
        if isinstance(expr, int):
            yield [expr, depth]
        else:
            yield from flatten(expr[0], depth + 1)
            yield from flatten(expr[1], depth + 1)

    def halves(arr):
        """Split depth list into the two halves."""
        acc = 0
        for (i, (_, d)) in enumerate(arr):
            # Add elements up to 1, weighting them less the deeper they
            # are. The formula is set up so that 1 is reached at half of
            # array.
            acc += Fraction(1 / 2 ** (d - 1))
            if acc > 1:
                return (
                    [[x, d - 1] for x, d in arr[:i]],
                    [[x, d - 1] for x, d in arr[i:]],
                )
        assert False, "Shouldn't get here"

    def rebuild(arr):
        """Build a depth list back into an expr."""

        if len(arr) == 1:
            return arr[0][0]
        else:
            a, b = halves(arr)
            return [rebuild(a), rebuild(b)]

    arr = list(flatten(expr))

    for (i, (x, depth)) in enumerate(arr):
        if depth > 4:
            # Explode
            a = arr[i][0]
            b = arr[i + 1][0]

            # Turn pair into a single zero.
            arr[i][0] = 0
            arr[i][1] -= 1
            del arr[i + 1]

            # Grow number to left, if any.
            if i - 1 >= 0:
                arr[i - 1][0] += a

            # Grow number to right, if any.
            if i + 1 < len(arr):
                arr[i + 1][0] += b

            # Step done, exit.
            return rebuild(arr)

    for (i, (x, depth)) in enumerate(arr):
        if x >= 10:
            # Split
            arr[i] = [x // 2, depth + 1]
            arr.insert(i + 1, [(x + 1) // 2, depth + 1])

            # Step done, exit.
            return rebuild(arr)

    return rebuild(arr)


def add(a, b):
    expr = [a, b]
    while True:
        new_expr = step_reduce(expr)
        if new_expr == expr:
            return expr
        else:
            expr = new_expr


def abs(expr):
    if isinstance(expr, int):
        return expr
    else:
        return 3 * abs(expr[0]) + 2 * abs(expr[1])


if __name__ == "__main__":
    data = read(eval)

    # 1
    sum = reduce(add, data)
    print(abs(sum))

    # 2
    print(max(abs(add(i, j)) for i in data for j in data if i != j))


def p(expr):
    """Fancy visualizer function."""
    def q(expr, depth):
        if isinstance(expr, int):
            if expr >= 10 and depth <= 4:
                eprint("\x1b[1;35m%s\x1b[0m" % expr, end="")
            else:
                eprint(expr, end="")
        else:
            if depth == 4:
                eprint("\x1b[1;31m", end="")
            eprint("[", end="")
            q(expr[0], depth + 1)
            eprint(", ", end="")
            q(expr[1], depth + 1)
            eprint("]", end="")
            if depth == 4:
                eprint("\x1b[0m", end="")

    q(expr, 0)
    eprint()
