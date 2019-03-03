 # Approach to Limbo (Part 1)

- He starts at 1, so this one does not really need to be considered.
- Key part is that the order in which he moves left / right does not matter. So I just need to do it all in one direction first, then in the other direction.
- Also, keep in mind is that the final result may be too large to store in a 32-bit integer.

## Formula for Leftmost Integers

- If he moves left all the time, the numbers which he reaches form the following sequence:

  - `1, 2, 4, 7, 11, ...`

  - 1st difference: `1, 2, 3, 4, ...`

  - 2nd difference: `1`

  - The base equation is the following (where n is the row number):

$$
f(n) = an^2 + bn + c\\
a = 1/2\\
\therefore f(n) = \frac{1}{2}n^2 + bn + c
$$

  - Continuing this working out:

$$
f(0) = c = 1\\

f(1) = \frac{1}{2} + b + c = 2\\
b + c = \frac{3}{2}
$$

- Solving:

$$
b = \frac{1}{2}\\
c = 1
$$

- So final equation for the leftmost integers is:

$$
f(n) = \frac{1}{2}n^2 + \frac{1}{2}n + 1
$$

When using this formula, n should be the row number (with the first row being row 0) moved down a row (direction does not matter). E.g. if he moves left 1 time and right 2 times, `n = 3`, which is the 4th row in the diagram.

Be careful when implementing this; the sequence terms are always integers, so divide last to prevent any rounding errors with floats. Something like this should be implemented instead:
$$
f(n) = \frac{1}{2}(n^2 + n + 2)
$$

## Moving to Right (from leftmost integer)

Once we have the leftmost integer, we just have to add the number of times the person moves to the right to the number obtained from the formula. This should give us the appropriate value.

E.g. in one of the samples:

- He moves left 1 time and right 2 times. This gives us `n = 3`, and the value is 7 in this case.
- Since he moved right 2 times, `7 + 2 = 9`, which is the desired answer.

## Pseudocode

```
INPUT leftMoves, rightMoves
LET n <- leftMoves + rightMoves
LET leftMostInteger = (n^2 + n + 2) / 2
OUTPUT leftMostInteger + rightMoves
```

