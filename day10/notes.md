Example:

[.####.#.] (3,4,5,7) (2,4,5,6,7) (1,4,7) (1,3,4,7) (1,2,3,4,5,7) (7) (1,2,3,6) (0,1,3,6,7) {4,59,39,250,242,220,26,250}

In joltages there are 4 larger numbers at indices (3,4,5,7). This matches the first button.

Another idea is to look at the min presses. Here 0 must be bumped 4 times. Which means (0,1,3,6,7) will be pressed 4 times.

Another example:

[.##...#.] (4,7) (2,5,7) (0,1,4,6,7) (1,4) (0,3,4,5,6,7) (0,1,2,3,4,5,7) (0,1,2,3,4,6) (2,3,5,6,7) {39,35,37,41,50,41,32,59}

Not so much larger than others. Maybe all but 6 ((0,1,2,3,4,5,7)) works?

Smallest... joltage 6 (32). Many options to press it.

Another:

[.##...] (1,3,4) (0,1,3) (3,5) (0) (0,1,2,5) (2,3,4) {197,208,200,41,31,185}

0,1,2,5 are much larger than the rest.. Should we prefer it?

Another:

[#####.##.] (1,5,8) (1,2,3,5,6) (2,4,8) (3,4,8) (2,4) (0,1,7,8) (2,3,6) (2,3,7,8) (4,7) {1,181,199,203,34,180,179,12,45}

1,2,3,5,6  are much larger... prefer it?

Here joltage 0 is only bumped once. Which means (0,1,7,8) is pressed once and never considered again.
Then at pos 7 (12)


# Greedy?

What if we find the largest common button and press it for the max possible times?

Couldn't we skip a better solution? Yes...


# Multiple magic?

(3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

joltage 0 press 3 times
joltage 1 press 5 times
joltage 2 press 4 times
joltage 3 press 7 times

To bump joltage 0 we have two options:
- (0,1)
- (0,2)

We for sure need to press 3 times among these two buttons. Either
- 3 x 0
- 2 x 1
- 1 x 2
- 0 x 3

Either way, this is a short cut in the bfs.
