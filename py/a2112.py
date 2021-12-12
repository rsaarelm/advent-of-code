from collections import defaultdict
from prelude import read


def explore(graph, pos="start", seen=set(), revisit=False):
    for step in graph[pos]:
        if step == "end":
            yield 1
        elif revisit or step not in seen:
            new_seen = set(seen)
            if step != "end" and step.islower():
                new_seen.add(step)
            yield from explore(
                graph, step, new_seen, revisit=revisit and step not in seen
            )


if __name__ == "__main__":
    data = read(lambda c: c.split("-"))
    graph = defaultdict(set)
    for (a, b) in data:
        # No returning to start, no leaving end.
        if b != "start" and a != "end":
            graph[a].add(b)
        if a != "start" and b != "end":
            graph[b].add(a)
    print(sum(explore(graph)))
    print(sum(explore(graph, revisit=True)))
