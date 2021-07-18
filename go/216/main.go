package main

import "fmt"

func main() {
	k1, n1 := 3, 7
	k2, n2 := 3, 9
	k3, n3 := 4, 1
	k4, n4 := 2, 18

	// Expecting [[1, 2, 4]]
	fmt.Printf("%v\n", combinationSum3(k1, n1))

	// Expecting [[1, 2, 6], [1, 3, 5], [2, 3, 4]]
	fmt.Printf("%v\n", combinationSum3(k2, n2))

	// Expecting []
	fmt.Printf("%v\n", combinationSum3(k3, n3))

	// Expecting []
	fmt.Printf("%v\n", combinationSum3(k4, n4))
}

func combinationSum3(k, n int) [][]int {
	var ret [][]int

	var stack []StackStuff
	var trace []int
	sum := 0
	stack = append(stack, StackStuff{
		startIdx: 1,
		endIdx:   min(n+1, 10),
	})

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

		sum += ss.startIdx
		trace = append(trace, ss.startIdx)
		ss.startIdx++

		if len(trace) == k {
			if sum == n {
				ret = append(ret, append([]int(nil), trace...))
			}

			sum -= trace[len(trace)-1]
			trace = trace[:len(trace)-1]
		} else {
			endIdx := n - sum + 1
			stack = append(stack, StackStuff{
				startIdx: ss.startIdx,
				endIdx:   min(endIdx, 10),
			})
		}
	}

	return ret
}

type StackStuff struct {
	startIdx int
	endIdx   int
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
