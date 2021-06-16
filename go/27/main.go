package main

import "fmt"

func main() {
	nums1, val1 := []int{3, 2, 2, 3}, 3
	nums2, val2 := []int{0, 1, 2, 2, 3, 0, 4, 2}, 2

	// Expecting 2, [2, 2]
	ret1 := removeElement(nums1, val1)
	fmt.Printf("%v %v\n", ret1, nums1[0:ret1])

	// Expecting 5, [0, 1, 4, 0, 3]
	ret2 := removeElement(nums2, val2)
	fmt.Printf("%v %v\n", ret2, nums2[0:ret2])
}

func removeElement(nums []int, val int) int {
	numRemoved := 0
	slow, fast := 0, 0
	oriLen := len(nums)

	for ; fast < oriLen; fast++ {
		if nums[fast] != val {
			nums[slow] = nums[fast]
			slow++
		} else {
			numRemoved++
		}
	}

	return oriLen - numRemoved
}
