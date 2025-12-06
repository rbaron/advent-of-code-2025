import fileinput
import re
from functools import reduce


PATT = re.compile(r"\S+")


def parse():
    rows = []
    for row in open("input.txt").readlines():
        rows.append(PATT.findall(row))
    cols = []
    for x in range(len(rows[0])):
        cols.append([rows[y][x] for y in range(len(rows))])
    return cols


def eval(stack):
    op, *ns = stack
    ns = map(int, ns)
    match op:
        case "+":
            return reduce(lambda a, b: a + b, ns)
        case "*":
            return reduce(lambda a, b: a * b, ns)


def parse_pt2():
    with open("input.txt") as f:
        rows = list(f.read().splitlines())
    cols = []
    new_col = []
    for x in range(len(rows[0])):
        if rows[-1][x] in "+*":
            new_col.append(rows[-1][x])
        if not any(r[x] != " " for r in rows):
            cols.append(new_col)
            new_col = []
            continue
        new_col.append("".join(r[x] for r in rows if r[x].isdigit()))

    cols.append(new_col)
    return cols


print(sum(eval(reversed(s)) for s in parse()))
print(sum(eval(s) for s in parse_pt2()))
