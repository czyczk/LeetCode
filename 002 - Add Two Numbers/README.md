You are given two **non-empty** linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

**Example:**

```
Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
```

**Tips & Traps:**

1. The input non-negative integers are in reverse order, so the integers to be added in the first place
just naturally come in the first.

2. Carry can remain while the last sum comes out. Remember to add it to the result. 


**Best solution:**

1. Use 1 and 0 for the carry instead of *true* or *false*, since `sum` can be represented directly as
`num1 + num2 + carry` in this way.

2. The direct number to decide what to add into the result is `sum % 10` and
the carry is directly represented as `sum / 10`. There's no need to adopt extra if/else structures.

