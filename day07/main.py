from functools import cache


def parse():
    with open("input.txt") as f:
        return {
            (x, y): c
            for y, l in enumerate(f.readlines())
            for x, c in enumerate(l)
            if c in "S^"
        }


def step(beams, grid):
    n_splits = 0
    new_beams = set()
    for x, y in beams:
        c = g.get((x, y + 1), ".")
        if c == "^":
            n_splits += 1
            new_beams |= set([(x - 1, y + 1), (x + 1, y + 1)])
        else:
            new_beams.add((x, y + 1))
    return new_beams, n_splits


def count_paths(x, y, g, H):
    @cache
    def inner(x,y):
        if y >= H:
            return 1
        match g.get((x, y)):
            case "^":
                return inner(x - 1, y + 1) + inner(x + 1, y)
            case _:
                return inner(x, y + 1)

    return inner(x, y)


g = parse()
H = max(y + 1 for x, y in g)
x, y = next(p for p, c in g.items() if c == "S")

beams = {(x, y)}
total_splits = 0
for _ in range(H):
    beams, n = step(beams, g)
    total_splits += n

print(total_splits)

print(count_paths(x, y, g, H))
