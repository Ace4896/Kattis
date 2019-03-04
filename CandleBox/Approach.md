# Approach to Candle Box

Another sequence related problem, but with 2 sequences this time.

Things to keep in mind:

- Rita is always at least 4 years old, due to the constraint on the candle number in her box.
- It is possible for Theo to not be 3 years old (and thus have no candles in his box) - in this case, the output is trivial. Just ensure that Theo is less than 3 years old first.
- Combining the sequences together might do the trick here.

## Combining the Two Sequences

### Rita's Candles

Rita's candles would be part of this series (where `n` is her age at her most recent birthday):
$$
S_n = 4 + 5 + 6 + ... + n
$$
Which gives us the following sequence (where `n >= 4`):
$$
a_n = 4, 9, 15, 22, ...
$$
The sum though is simply the regular sum to n, minus 1 + 2 + 3 = 6. So the formula for the elements in the sequence is simply the following:
$$
a_n = \frac{n(n+1)}{2} - 6 = \frac{1}{2}(n^2 + n - 12)
$$

### Theo's Candles

Theo's candles would be part of this series (where `m = n - D, m >= 3, D >= 1`):
$$
S_m = 3 + 4 + 5 + 6 + ... + m
$$
Which gives us this sequence:
$$
b_m = 3, 7, 12, 18, ...
$$
The sum of this sequence, like for Rita's candles, is also a regular sum to n, but minus 1 + 2 = 3 this time round. So the formula for the elements in the sequence is simply the following (substitute `m` for `n - D` in the formula:
$$
b_m = \frac{m(m+1)}{2} - 3 = \frac{1}{2}(m^2 + m - 6)
$$

### Combining Candles

When combining all candles in the same box, we get the following sequence:
$$
c_n = \frac{1}{2}(n^2 + n - 12) + \frac{1}{2}(m^2 + m - 6) = \frac{1}{2}(n^2 + n + m^2 + m - 18)
$$
Converting the `m` parts into `n`:
$$
m^2 + m = (n - D)^2 + (n - D)= n^2 -2nD + D^2 + n - D\\
= n^2 - n(2D - 1) + D^2 - D
$$
So we get a final formula of the following:
$$
c_n = \frac{1}{2}(2n^2 - n(2D - 2) + D^2 - D - 18)
$$
`D` and `cn` are given, so now re-arranging in terms of `n`:
$$
2c_n = 2n^2 - 2n(D - 1) + D^2 - D - 18\\
2n^2 - 2n(D - 1) + D^2 - D - 18 - 2c_n = 0\\
n = \frac{-b + \sqrt{b^2 - ac}}{a}\\
a = 2\\
b = - (D - 1) = 1 - D\\
c = D^2 - D - 18 - 2c_n
$$
*(Note that the negative answer to the quadratic is ignored, as otherwise Rita may have a negative age).*

This will allow us to find Rita's age, which we can then use to check Theo's age.

## Final Calculations

Once we have Rita's age `n`, we now have two cases for Theo's age:

1. Theo is younger than 3 years old
2. Theo is at least 3 years old

### Case 1 - Younger Than 3 Years Old

If Theo is younger than 3 years old, then no candles have to be transferred - the output is 0, since he should not have any candles at this point.

### Case 2 - At Least 3 Years Old

If Theo is at least 3 years old, then he should have some candles in his box. Substitute the value of `n` into the formula for Rita's candles only, and get the difference, i.e.:
$$
\text{Candles to Transfer} = \text{Actual Candles for Rita} - a_n
$$
