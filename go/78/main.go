package main

import "fmt"

func main() {
	n1 := []int{1, 2, 3}
	n2 := []int{0}

	// Expecting [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]]
	fmt.Printf("%v\n", subsets(n1))
	// Expecting [[], [0]]
	fmt.Printf("%v\n", subsets(n2))
}

var ans [][]int
var trace []int

func subsets(nums []int) [][]int {
	ans = [][]int{{}}
	backtraceRec(nums, 0)

	return ans
}

func backtraceRec(nums []int, startIdx int) {
	if startIdx == len(nums) {
		return
	}

	for i := startIdx; i < len(nums); i++ {
		trace = append(trace, nums[i])
		ans = append(ans, append([]int{}, trace...))
		backtraceRec(nums, i+1)
		trace = trace[:len(trace)-1]
	}
}
