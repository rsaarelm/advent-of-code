from prelude import input_str, ints, re_match, eprint

def rule(fold):
    axis, n = fold
    if axis == 'x':
        return lambda p: (n - (p[0] - n), p[1]) if p[0] > n else p
    elif axis == 'y':
        return lambda p: (p[0], n - (p[1] - n)) if p[1] > n else p

def comp(f1, f2):
    return lambda a: f1(f2(a))

if __name__ == "__main__":
    points, folds = input_str().split("\n\n")
    points = [ints(line) for line in points.splitlines()]
    folds = [
        re_match(r"fold along (.)=(.+)", line) for line in folds.splitlines()
    ]

    # 1
    print(len(set(rule(folds[0])(p) for p in points)))

    # 2
    f = lambda x: x
    for fold in folds:
        f = comp(rule(fold), f)

    folded = set(f(p) for p in points)

    for y in range(7):
        for x in range(42):
            if (x, y) in folded:
                eprint('██', end='')
            else:
                eprint(' .', end='')
        eprint()

# Lookup...
# if y > 7, y = 7 - y
