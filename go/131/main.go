package main

import "fmt"

func main() {
	s1 := "aab"

	// Expecting [["a", "a", "b"], ["aa", "b"]]
	fmt.Printf("%v\n", partition(s1))
}

func partition(s string) [][]string {
	n := len(s)
	dp := make([][]bool, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]bool, n)
	}

	for j := 0; j < n; j++ {
		for i := 0; i <= j; i++ {
			if j-i <= 2 && s[i] == s[j] {
				dp[i][j] = true
			} else {
				dp[i][j] = dp[i+1][j-1] && s[i] == s[j]
			}
		}
	}

	var ret [][]string

	var stack []StackStuff
	stack = append(stack, StackStuff{
		startIdx: 0,
		endIdx:   0,
	})
	var trace []StackStuff

	for len(stack) != 0 {
		lenStack := len(stack)
		ss := stack[lenStack-1]
		stack[lenStack-1].endIdx++
		if ss.endIdx == n {
			stack = stack[:lenStack-1]
			if len(trace) == 0 {
				break
			}

			trace = trace[:len(trace)-1]
			continue
		}

		if dp[ss.startIdx][ss.endIdx] {
			trace = append(trace, ss)
			if ss.endIdx == n-1 {
				// all true
				var strs []string
				for _, idxPair := range trace {
					strs = append(strs, s[idxPair.startIdx:idxPair.endIdx+1])
				}
				ret = append(ret, strs)

				trace = trace[:len(trace)-1]
			} else {
				stack = append(stack, StackStuff{
					startIdx: ss.endIdx + 1,
					endIdx:   ss.endIdx + 1,
				})
			}
		}
	}

	return ret
}

type StackStuff struct {
	startIdx int // inclusive
	endIdx   int // inclusive
}
