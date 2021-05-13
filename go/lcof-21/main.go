package main

import "fmt"

func main() {
	n1 := []int{1, 2, 3, 4}
	n2 := []int{1}
	n3 := []int{1, 4, 4, 2, 3, 5, 2, 6, 3, 3}

	// Expecting [1, 3, 2, 4]
	fmt.Printf("%v\n", exchange(n1))
	// Expecting [1]
	fmt.Printf("%v\n", exchange(n2))
	// Expecting [1, 3, 3, 3, 5, 2, 2, 4, 4, 6]
	fmt.Printf("%v\n", exchange(n3))
}

func exchange(nums []int) []int {
	n := len(nums)
	if n == 0 {
		return nums
	}

	// Find the first even number
	i := 0
	for ; i < n; i++ {
		if nums[i]&1 == 0 {
			break
		}
	}

	if i == n-1 {
		return nums
	}

	// Find the first odd number in reverse order
	j := n - 1
	for ; j >= 0; j-- {
		if nums[j]&1 == 1 {
			break
		}
	}

	if j <= i {
		return nums
	}

	for i < n && j > i {
		nums[i], nums[j] = nums[j], nums[i]

		for i < n && nums[i]&1 == 1 {
			i++
		}

		for j > i && nums[j]&1 == 0 {
			j--
		}
	}

	return nums
}
