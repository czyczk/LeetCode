package main

import "fmt"

func main() {
	nums1 := []int{1, 3, -1, -3, 5, 3, 6, 7}
	k1 := 3
	nums2 := []int{7, 2, 4}
	k2 := 2

	// Expecting [3, 3, 5, 5, 6, 7]
	fmt.Println(maxSlidingWindow(nums1, k1))
	// Expecting [7, 4]
	fmt.Println(maxSlidingWindow(nums2, k2))
}

func maxSlidingWindow(nums []int, k int) []int {
	n := len(nums)

	var idxList []int
	result := make([]int, n-k+1)

	for i, num := range nums {
		/*
		 * 1. Push back the current index if 1.1 || 1.2.
		 *   1.1 idxList is empty.
		 *   1.2 the number indicated by the last in idxList is smaller than the current one.
		 * 2. Pop all the numbers <= the current one and add the index.
		 */
		originalLen := len(idxList)
		reducedLen := originalLen
		for reducedLen != 0 && num >= nums[idxList[reducedLen-1]] {
			reducedLen--
		}

		if reducedLen != originalLen {
			idxList = idxList[:reducedLen]
		}
		idxList = append(idxList, i)

		// Just move ahead if not enough numbers have been iterated, i.e. a window is not formed.
		if i < k-1 {
			continue
		}

		// Check if the head in idxList is in the window range. Pop the head if it's out of range.
		// head < l
		if idxList[0] < i+1-k {
			idxList = idxList[1:]
		}

		// Now the head indicates the index of the max number within the window.
		result[i+1-k] = nums[idxList[0]]
	}

	return result
}
