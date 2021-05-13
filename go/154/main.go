package main

import "fmt"

func main() {
	n1 := []int{1, 3, 5}
	n2 := []int{2, 2, 2, 0, 1}
	n3 := []int{4, 5, 6, 7, 0, 1, 4}

	// Expecting 1
	fmt.Println(findMin(n1))

	// Expecting 0
	fmt.Println(findMin(n2))

	// Expecting 0
	fmt.Println(findMin(n3))
}

func findMin(nums []int) int {
	n := len(nums)
	if n == 1 {
		return nums[0]
	}

	first := nums[0]
	last := first

	for i := 1; i < n; i++ {
		num := nums[i]
		if num < last {
			return num
		}

		last = num
	}

	return first
}
