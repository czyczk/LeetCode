package main

import (
	"fmt"
	"sort"
)

func main() {
	n1 := []int{1, 2, 2}
	n2 := []int{0}

	// Expecting [[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]
	fmt.Printf("%v\n", subsetsWithDup(n1))
	// Expecting [[], [0]]
	fmt.Printf("%v\n", subsetsWithDup(n2))
}

var ans [][]int
var trace []int

func subsetsWithDup(nums []int) [][]int {
	ans = [][]int{{}}
	trace = []int{}
	sort.Ints(nums)
	backtraceRec(nums, 0)
	return ans
}

func backtraceRec(nums []int, startIdx int) {
	if startIdx == len(nums) {
		return
	}

	var lastNum int
	for i := startIdx; i < len(nums); i++ {
		if i != startIdx && nums[i] == lastNum {
			continue
		}

		trace = append(trace, nums[i])
		lastNum = nums[i]
		ans = append(ans, append([]int{}, trace...))
		backtraceRec(nums, i+1)
		trace = trace[:len(trace)-1]
	}
}
