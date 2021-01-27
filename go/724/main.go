package main

import "fmt"

func main() {
	nums1 := []int{1, 7, 3, 6, 5, 6}
	nums2 := []int{1, 2, 3}

	// Expecting 3
	fmt.Println(pivotIndex(nums1))
	// Expecting -1
	fmt.Println(pivotIndex(nums2))
}

func pivotIndex(nums []int) int {
	n := len(nums)
	if n == 0 {
		return -1
	} else if n == 1 {
		return 0
	} else if n == 2 {
		if nums[0] == 0 {
			return 1
		} else if nums[1] == 0 {
			return 0
		} else {
			return -1
		}
	}

	leftSum := 0
	rightSum := 0

	for i := 1; i < n; i++ {
		rightSum += nums[i]
	}

	for i := 0; i < n; i++ {
		if leftSum == rightSum {
			return i
		}

		if i < n-1 {
			leftSum += nums[i]
			rightSum -= nums[i+1]
		}
	}

	if leftSum == rightSum {
		return n - 1
	}

	return -1
}
