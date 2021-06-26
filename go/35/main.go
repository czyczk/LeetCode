package main

func main() {
	n1, t1 := []int{1, 3, 5, 6}, 5
	n2, t2 := []int{1, 3, 5, 6}, 2
	n3, t3 := []int{1, 3, 5, 6}, 7
	n4, t4 := []int{1}, 0

	// Expecting 2
	println(searchInsert(n1, t1))
	// Expecting 1
	println(searchInsert(n2, t2))
	// Expecting 4
	println(searchInsert(n3, t3))
	// Expecting 0
	println(searchInsert(n4, t4))
}

func searchInsert(nums []int, target int) int {
	n := len(nums)
	if n == 1 {
		if target <= nums[0] {
			return 0
		} else {
			return 1
		}
	}

	// Both inclusive
	start, end := 0, n-1
	for start <= end {
		idx := (start + end) / 2
		if nums[idx] == target {
			return idx
		} else if nums[idx] < target {
			start = idx + 1
		} else {
			end = idx - 1
		}
	}

	return start
}
