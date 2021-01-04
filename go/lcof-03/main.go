package main

import "fmt"

func main() {
	nums1 := []int{2, 3, 1, 0, 2, 5, 3}

	// Expecting 2 or 3
	fmt.Println(findRepeatNumber(nums1))
}

func findRepeatNumber(nums []int) int {
	n := len(nums)

	for i := 0; i < n; i++ {
		num := nums[i]
		for i != num {
			if num == nums[num] {
				return num
			}

			nums[i] = nums[num]
			nums[num] = num
		}
	}

	return -1
}
