from functools import cache
from prelude import read, ints


@cache
def zero_fish_spawns_in(days):
    if days < 0:
        return 0
    else:
        return (
            1 + zero_fish_spawns_in(days - 7) + zero_fish_spawns_in(days - 9)
        )


if __name__ == "__main__":
    data = read(ints)[0]

    print(sum(1 + zero_fish_spawns_in(79 - x) for x in data))
    print(sum(1 + zero_fish_spawns_in(255 - x) for x in data))
