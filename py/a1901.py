import sys

# 1
def fuel_1(mass):
    return mass // 3 - 2

# 2
def fuel_2(mass):
    acc = 0
    fuel = fuel_1(mass)
    while fuel >= 0:
        acc += fuel
        fuel = fuel_1(fuel)
    return acc

if __name__ == '__main__':
    masses = [int(line) for line in sys.stdin]
    print(sum(map(fuel_1, masses)))
    print(sum(map(fuel_2, masses)))
