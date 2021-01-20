package main

import (
	"fmt"
	"math"
)

func main() {
	nums1 := []int{1, 2, 3}
	nums2 := []int{1, 2, 3, 4}
	nums3 := []int{-1, -2, -3}
	nums4 := []int{-100, -98, -1, 2, 3, 4}
	nums5 := []int{6, 7, 0, -2}

	// Expecting 6
	fmt.Println(maximumProduct(nums1))
	// Expecting 24
	fmt.Println(maximumProduct(nums2))
	// Expecting -6
	fmt.Println(maximumProduct(nums3))
	// Expecting 39200
	fmt.Println(maximumProduct(nums4))
	// Expecting 0
	fmt.Println(maximumProduct(nums5))
}

func maximumProduct(nums []int) int {
	max1 := math.MinInt32
	max2 := math.MinInt32
	max3 := math.MinInt32
	min1 := math.MaxInt32
	min2 := math.MaxInt32

	for _, num := range nums {
		if num > max1 {
			max3 = max2
			max2 = max1
			max1 = num
		} else if num > max2 {
			max3 = max2
			max2 = num
		} else if num > max3 {
			max3 = num
		}

		if num < min1 {
			min2 = min1
			min1 = num
		} else if num < min2 {
			min2 = num
		}
	}

	return max(max1*max2*max3, min1*min2*max1)
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
