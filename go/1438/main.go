package main

import (
	"fmt"
)

func main() {
	nums1 := []int{8, 2, 4, 7}
	limit1 := 4

	nums2 := []int{10, 1, 2, 4, 7, 2}
	limit2 := 5

	nums3 := []int{4, 2, 2, 2, 4, 4, 2, 2}
	limit3 := 0

	nums4 := []int{1, 5, 6, 7, 8, 10, 6, 5, 6}
	limit4 := 4

	nums5 := []int{38, 73, 69, 15, 59, 36, 14, 6, 38, 2, 79, 86, 2, 12, 53, 15, 6, 25, 31, 76, 54, 21, 15, 58, 22, 88, 31, 21, 96, 14, 56, 49, 70, 38, 71, 33, 92, 62, 41, 13, 27, 84, 41, 6, 4, 2, 38, 93, 77, 41, 58, 51, 41, 52, 9, 9, 41, 77, 59, 15, 33, 28, 80, 100, 70, 89, 61}
	limit5 := 73

	// Expecting 2
	fmt.Println(longestSubarray(nums1, limit1))
	// Expecting 4
	fmt.Println(longestSubarray(nums2, limit2))
	// Expecting 3
	fmt.Println(longestSubarray(nums3, limit3))
	// Expecting 5
	fmt.Println(longestSubarray(nums4, limit4))
	// Expecting 15
	fmt.Println(longestSubarray(nums5, limit5))
}

func longestSubarray(nums []int, limit int) int {
	left, right := 0, 0
	minQ, maxQ := []int{}, []int{}

	n := len(nums)
	maxWindowSize := 1
	for right < n {
		num := nums[right]

		for len(minQ) > 0 && minQ[len(minQ)-1] > num {
			minQ = minQ[:len(minQ)-1]
		}
		minQ = append(minQ, num)
		for len(maxQ) > 0 && maxQ[len(maxQ)-1] < num {
			maxQ = maxQ[:len(maxQ)-1]
		}
		maxQ = append(maxQ, num)

		for maxQ[0]-minQ[0] > limit {
			leftNum := nums[left]
			if maxQ[0] == leftNum {
				maxQ = maxQ[1:]
			}
			if minQ[0] == leftNum {
				minQ = minQ[1:]
			}
			left++
		}

		maxWindowSize = max(maxWindowSize, right-left+1)
		right++
	}

	return maxWindowSize
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
