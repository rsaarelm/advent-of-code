from prelude import *
from z3 import *

# TODO: Make this format the output properly...

if __name__ == '__main__':
    x = Int('x')
    y = Int('y')
    z = Int('z')
    dx = Int('dx')
    dy = Int('dy')
    dz = Int('dz')

    s = Solver()

    for i, line in enumerate(read()):
        (px, py, pz, vx, vy, vz) = ints(line)
        s.add(px + Int('n_%s' % i) * (vx - dx) == x)
        s.add(py + Int('n_%s' % i) * (vy - dy) == y)
        s.add(pz + Int('n_%s' % i) * (vz - dz) == z)

    if s.check() == sat:
        m = s.model()
        eprint("P:", m[x], m[y], m[z])
        eprint("V:", m[dx], m[dy], m[dz])
    else:
        eprint("No solution")

