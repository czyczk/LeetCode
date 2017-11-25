Given an array of integers, return **indices** of the two numbers such that they add up to a specific target.

You may assume that each input would have ***exactly*** one solution, and you may not use the *same* element twice.

**Example:**

```
Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
```

**Traps:**

Numbers can be negative.

First assumed all numbers are positive which was a trap itself.

Because when they're all positive,
numbers that are larger than the target can be ignored since
it'll be bigger if added with another number. But this does not
apply to the negative ones.

**Best solution:**

$n$. In the previous solution, the time complexity is $n^2$.

The worst thing is that the visited numbers are forgotten.

E.g. when i == 1, (nearly) all the numbers are iterated once through j,
but the same thing happens again when i == 2.

Actually, visited numbers can be "cached" along with its position in a
query-friendly data structure: HashMap.

When we are visiting a number, we are actually finding its complement. If
the visited numbers are cached, we can easily find it in $1$. And since
going though all the numbers costs $n$ of time, the best time complexity is
$n$.