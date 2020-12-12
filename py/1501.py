import sys

input = sys.stdin.read()

d = {'(': 1, ')': -1}

# 1

floor = 0
for c in input:
    floor += d[c]
print(floor)

# 2

floor = 0
for (i, c) in enumerate(input):
    floor += d[c]
    if floor < 0:
        print(i + 1)
        break
