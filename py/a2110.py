from functools import reduce
from prelude import read

LEFT, RIGHT = "([{<", ")]}>"


def analyze(line):
    stack = []
    mismatch = []
    for c in line:
        if c in LEFT:
            stack.append(RIGHT[LEFT.index(c)])
        else:
            d = stack.pop()
            if d != c:
                mismatch.append(c)
    return ("".join(mismatch), "".join(reversed(stack)))


if __name__ == "__main__":
    data = read()

    score_1 = 0
    scores_2 = []
    for line in data:
        (mismatch, completion) = analyze(line)
        if mismatch:
            for c in mismatch:
                score_1 += [3, 57, 1197, 25137][RIGHT.index(c)]
        else:
            scores_2.append(
                reduce(
                    lambda a, b: a * 5 + b,
                    (RIGHT.index(c) + 1 for c in completion),
                )
            )

    # 1
    print(score_1)

    # 2
    scores_2.sort()
    print(scores_2[len(scores_2) // 2])
