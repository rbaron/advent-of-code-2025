# Input shape
- Only 5 shapes
- All presents are 3x3
- Either 2 or 3 empty spaces

# Tree space
e.g.:

40x46: 39 54 54 49 46 41

- Trying every combination of flip/mirror looks unfeasible.
- All spaces require similar number of each presents


# Shortcuts
- If sum of occupied gifts slots > space size, bail
  - I tried it and 425 of the 1000 areas fit... Suspicious... It turned out to be the correct answer... :(
- Rorating / flipping will yield the same shape sometimes. We can deduplicate
  - When?
  - Just precompute unique?


# Ideas
- Greedy?
- Pack a small area as tightly as possible and make that a "unit"?
  - Find the smallest area I can place one of each present
- Build from bottom up
  - (H: 0, W: 0)
    - (0 0 0 0 0) -> True
    - (0 0 0 0 1) -> False
    - (0 0 0 0 2) -> False
    - ...
  - (H: 0, W: 1)
    - (0 0 0 0 0) -> True
    - (0 0 0 0 1) -> False
