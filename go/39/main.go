package main

import (
	"fmt"
	"sort"
)

func main() {
	c1, t1 := []int{2, 3, 6, 7}, 7
	c2, t2 := []int{2, 3, 5}, 8
	c3, t3 := []int{2}, 1
	c4, t4 := []int{1}, 1
	c5, t5 := []int{1}, 2
	c6, t6 := []int{2, 7, 6, 3, 5, 1}, 9

	// Expecting [[2, 2, 3], [7]]
	fmt.Printf("%v\n", combinationSum(c1, t1))
	// Expecting [[2, 2, 2, 2], [2, 3, 3], [3, 5]]
	fmt.Printf("%v\n", combinationSum(c2, t2))
	// Expecting []
	fmt.Printf("%v\n", combinationSum(c3, t3))
	// Expecting [[1]]
	fmt.Printf("%v\n", combinationSum(c4, t4))
	// Expecting[[1, 1]]
	fmt.Printf("%v\n", combinationSum(c5, t5))
	// 21 results in total
	fmt.Printf("%v\n", len(combinationSum(c6, t6)))
}

func combinationSum(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	var stack [][]int
	var startIndexes []int
	{
		var q []int
		for _, c := range candidates {
			if c <= target {
				q = append(q, c)
			}
		}
		stack = append(stack, q)
		startIndexes = append(startIndexes, -1)
	}

	var trace []int
	sum := 0
	var ret [][]int

	for len(stack) != 0 {
		lenStack := len(stack)
		q := stack[lenStack-1]
		if len(q) == 0 {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}
			sum -= trace[len(trace)-1]
			trace = trace[:len(trace)-1]
			startIndexes = startIndexes[:len(startIndexes)-1]
			continue
		}

		c := q[0]
		stack[lenStack-1] = q[1:]
		trace = append(trace, c)
		sum += c
		startIndexes[len(startIndexes)-1]++
		//fmt.Printf("Used: %v. Trace: %v. Si: %v\n", c, trace, startIndexes)

		rem := target - sum
		if rem == 0 {
			ret = append(ret, append([]int{}, trace...))
			sum -= trace[len(trace)-1]
			trace = trace[:len(trace)-1]
		} else {
			var newQ []int
			for _, c := range candidates[startIndexes[len(startIndexes)-1]:] {
				if c > rem {
					break
				}
				newQ = append(newQ, c)
			}
			stack = append(stack, newQ)
			startIndexes = append(startIndexes, startIndexes[len(startIndexes)-1]-1)
			//fmt.Printf("NewQ: %v. NewSi:%v\n", newQ, startIndexes)
		}
	}

	return ret
}
