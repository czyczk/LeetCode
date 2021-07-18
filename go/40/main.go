package main

import (
	"fmt"
	"sort"
)

func main() {
	c1, t1 := []int{10, 1, 2, 7, 6, 1, 5}, 8
	c2, t2 := []int{2, 5, 2, 1, 2}, 5

	// Expecting [[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]]
	fmt.Printf("%v\n", combinationSum2(c1, t1))
	// Expecting [[1, 2, 2], [5]]
	fmt.Printf("%v\n", combinationSum2(c2, t2))
}

func combinationSum2(candidates []int, target int) [][]int {
	var ret [][]int
	sort.Ints(candidates)

	var stack []StackStuff
	{
		endIndex := 0
		for endIndex < len(candidates) {
			if candidates[endIndex] > target {
				break
			}

			endIndex++
		}

		stack = append(stack, StackStuff{
			startIdx: 0,
			endIdx:   endIndex,
			visited:  make(map[int]bool),
		})
	}

	var trace []int
	sum := 0

	for len(stack) != 0 {
		lenStack := len(stack)
		ss := &stack[lenStack-1]

		if ss.startIdx >= ss.endIdx {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}
			sum -= trace[len(trace)-1]
			trace = trace[:len(trace)-1]
			continue
		}

		c := candidates[ss.startIdx]
		ss.startIdx++
		if ss.visited[c] {
			continue
		}

		ss.visited[c] = true
		sum += c
		trace = append(trace, c)

		if sum == target {
			ret = append(ret, append([]int(nil), trace...))
			sum -= trace[len(trace)-1]
			trace = trace[:len(trace)-1]
		} else {
			endIdx := ss.startIdx
			for endIdx < len(candidates) {
				if candidates[endIdx] > target-sum {
					break
				}
				endIdx++
			}

			stack = append(stack, StackStuff{
				startIdx: ss.startIdx,
				endIdx:   endIdx,
				visited:  make(map[int]bool),
			})
		}
	}

	return ret
}

type StackStuff struct {
	startIdx int
	endIdx   int
	visited  map[int]bool
}
