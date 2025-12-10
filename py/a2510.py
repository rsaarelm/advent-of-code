from prelude import *
from z3 import *

MAX = 2**63

def p2(jolt, vecs):
    """Solve for one machine."""
    s = Optimize()
    n = len(jolt)
    m = len(vecs)

    # Coefficients for each vec. Find the ones that make vecs sum to jolt.
    vars = [Int('a_%s' % i) for i in range(m)]

    # Assert coefficients can't go negative, otherwise we get infinite
    # solutions.
    for v in vars:
        s.add(v >= 0)

    for (i, x) in enumerate(jolt):
        s.add(Sum(vars[j] for j in range(m) if i in vecs[j]) == x)

    s.minimize(Sum(vars))

    ret = MAX
    # Look for the smallest solution.
    if s.check() == sat:
        m = s.model()
        return sum(m[d].as_long() for d in m.decls())
    else:
        eprint("No solution!")

if __name__ == '__main__':
    machines = []
    for line in read():
        elts = line.split()
        lights = [i - 1 for (i, c) in enumerate(elts[0]) if c == '#']
        jolt = ints(elts[-1])
        vecs = [ints(x) for x in elts[1:-1]]
        machines.append((lights, vecs, jolt))

    # TODO P1
    print("-")

    print(sum(p2(m[2], m[1]) for m in machines))
