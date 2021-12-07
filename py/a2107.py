from prelude import read

if __name__ == "__main__":
    data = read(int, sep=",")

    a, b = min(data), max(data)

    # 1
    pos = -1
    cost = 999999999
    for i in range(a, b + 1):
        new_cost = sum(abs(x - i) for x in data)
        if new_cost < cost:
            pos = i
            cost = new_cost
    print(cost)

    # 2
    pos = -1
    cost = 999999999
    for i in range(a, b + 1):
        def dist_cost(n):
            return int(n * (n + 1) / 2)

        new_cost = sum(dist_cost(abs(x - i)) for x in data)
        if new_cost < cost:
            pos = i
            cost = new_cost
    print(cost)
