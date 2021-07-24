package main

import (
	"fmt"
	"sort"
)

func main() {
	n1 := []int{1, 1, 2}
	n2 := []int{1, 2, 3}

	// Expecting [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
	fmt.Printf("%v\n", permuteUnique(n1))
	// Expecting [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
	fmt.Printf("%v\n", permuteUnique(n2))
}

var ans [][]int
var trace []int
var used []bool

func permuteUnique(nums []int) [][]int {
	sort.Ints(nums)
	ans = [][]int{}
	trace = []int{}
	used = make([]bool, len(nums))

	backtraceRec(nums)
	return ans
}

func backtraceRec(nums []int) {
	if len(trace) == len(nums) {
		return
	}

	var lastNum int
	isFirstAdoption := true
	for i := 0; i < len(nums); i++ {
		if used[i] {
			continue
		}

		if !isFirstAdoption && nums[i] == lastNum {
			continue
		}

		lastNum = nums[i]
		isFirstAdoption = false

		trace = append(trace, nums[i])
		used[i] = true

		if len(trace) == len(nums) {
			ans = append(ans, append([]int{}, trace...))
		}
		backtraceRec(nums)

		trace = trace[:len(trace)-1]
		used[i] = false
	}
}
