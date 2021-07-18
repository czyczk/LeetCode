package main

import "fmt"

func main() {
	n1, k1 := 4, 2
	n2, k2 := 1, 1

	// Expecting [[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]]
	fmt.Printf("%v\n", combine(n1, k1))

	// Expecting [[1]]
	fmt.Printf("%v\n", combine(n2, k2))
}

func combine(n, k int) [][]int {
	var ret [][]int

	stack := []StackStuff{
		{
			startIdx: 0,
			endIdx:   n - k + 1,
		},
	}

	var trace []int

	for len(stack) != 0 {
		lenStack := len(stack)
		ss := &stack[lenStack-1]
		if ss.startIdx >= ss.endIdx {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}

			trace = trace[:len(trace)-1]
			continue
		}

		trace = append(trace, ss.startIdx+1)
		ss.startIdx++

		if len(trace) == k {
			ret = append(ret, append([]int(nil), trace...))
			trace = trace[:len(trace)-1]
		} else {
			stack = append(stack, StackStuff{
				startIdx: ss.startIdx,
				endIdx:   n - k + len(trace) + 1,
			})
		}
	}

	return ret
}

type StackStuff struct {
	startIdx int
	endIdx   int
}
