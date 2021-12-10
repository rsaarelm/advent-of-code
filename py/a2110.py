from prelude import read

PAIR = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}

SCORE = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137,
}

SCORE2 = {
    ")": 1,
    "]": 2,
    "}": 3,
    ">": 4,
}


def corruption(line):
    score = 0
    stack = []
    for c in line:
        if c in PAIR:
            stack.append(PAIR[c])
        else:
            if not stack or stack[-1] != c:
                score += SCORE[c]
            if stack:
                stack.pop()
    return score


def completion(line):
    stack = []
    for c in line:
        if c in PAIR:
            stack.append(PAIR[c])
        else:
            stack.pop()

    score = 0
    for c in reversed(stack):
        score *= 5
        score += " )]}>".index(c)
    return score


if __name__ == "__main__":
    data = read()

    # 1
    print(sum(corruption(line) for line in data))

    # 2
    valid = [line for line in data if not corruption(line)]
    scores = [completion(line) for line in valid]
    scores = list(sorted(c for c in scores if c))
    print(scores[len(scores) // 2])
