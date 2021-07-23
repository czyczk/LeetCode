package main

import "fmt"

func main() {
	n1 := []int{1, 2, 3}
	n2 := []int{0, 1}
	n3 := []int{1}

	// Expecting [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
	fmt.Printf("%v\n", permute(n1))

	// Expecting [[0, 1], [1, 0]]
	fmt.Printf("%v\n", permute(n2))

	// Expecting [[1]]
	fmt.Printf("%v\n", permute(n3))
}

var ans [][]int
var trace []int
var used []bool

func permute(nums []int) [][]int {
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

	for i := 0; i < len(nums); i++ {
		if used[i] {
			continue
		}

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
