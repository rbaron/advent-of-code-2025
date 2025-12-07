
def parse(filename):
    with open(filename, 'r') as file:
        contents = file.read()

    segs, ings = contents.split('\n\n')
    return [
        tuple(int(i) for i in seg.split('-'))
        for seg in segs.splitlines()
    ], [int(i) for i in ings.splitlines()]


segs, ings = parse("input.txt")

print(sum(1 for i in ings if any(s[0] <= i <= s[1] for s in segs)))

pts = sorted(
    p
    for (a, b) in segs
    for p in ((a, 'A'), (b, 'B'))
)

total = 0
seg_started_at = 0
a_count = 0
for p, ptype in pts:
    if ptype == 'A':
        if a_count == 0:
            seg_started_at = p
        a_count += 1
    else:
        a_count -= 1
        if a_count == 0:
            total += p - seg_started_at + 1
print(total)

