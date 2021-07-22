package main

import "fmt"

func main() {
	n1 := []int{4, 6, 7, 7}
	n2 := []int{4, 4, 3, 2, 1}
	n3 := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1}
	n4 := []int{100, 90, 80, 70, 60, 50, 60, 70, 80, 90, 100}

	// Expecting [[4, 6], [4, 6, 7], [4, 6, 7, 7], [4, 7], [4, 7, 7], [6, 7], [6, 7, 7], [7, 7]]
	fmt.Printf("%v\n", findSubsequences(n1))

	// Expecting [[4, 4]]
	fmt.Printf("%v\n", findSubsequences(n2))

	// Expecting 1018 items
	fmt.Printf("%v\n", len(findSubsequences(n3)))

	// Expecting 88 items
	fmt.Printf("%v\n", len(findSubsequences(n4)))
}

var ans [][]int
var trace []int

func findSubsequences(nums []int) [][]int {
	ans = [][]int{}
	trace = []int{}
	backtraceRec(nums, 0)
	return ans
}

func backtraceRec(nums []int, startIdx int) {
	if startIdx == len(nums) {
		return
	}

	used := make(map[int]bool)
	for i := startIdx; i < len(nums); i++ {
		if len(trace) > 0 && nums[i] < trace[len(trace)-1] {
			continue
		}

		if used[nums[i]] {
			continue
		}
		used[nums[i]] = true

		trace = append(trace, nums[i])
		if len(trace) >= 2 {
			ans = append(ans, append([]int{}, trace...))
		}

		backtraceRec(nums, i+1)
		trace = trace[:len(trace)-1]
	}
}
