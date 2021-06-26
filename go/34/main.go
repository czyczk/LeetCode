package main

import "fmt"

func main() {
	n1, t1 := []int{5, 7, 7, 8, 8, 10}, 8
	n2, t2 := []int{5, 7, 7, 8, 8, 10}, 6
	n3, t3 := []int{}, 0
	n4, t4 := []int{1}, 1

	// [3, 4]
	fmt.Printf("%v\n", searchRange(n1, t1))
	// [-1, -1]
	fmt.Printf("%v\n", searchRange(n2, t2))
	// [-1, -1]
	fmt.Printf("%v\n", searchRange(n3, t3))
	// [0, 0]
	fmt.Printf("%v\n", searchRange(n4, t4))
}

func searchRange(nums []int, target int) []int {
	n := len(nums)
	if n == 0 {
		return []int{-1, -1}
	}

	// Both inclusive
	start, end := 0, n-1
	for start <= end {
		idx := (start + end) / 2
		if nums[idx] == target {
			// Expand to the left
			startLeft, endLeft := start, idx
			for startLeft <= endLeft {
				idxLeft := (startLeft + endLeft) / 2
				if nums[idxLeft] == target {
					endLeft = idxLeft - 1
				} else {
					// nums[idxLeft] < target
					startLeft = idxLeft + 1
				}
			}
			idxLeft := startLeft

			// Expand to the right
			startRight, endRight := idx, end
			for startRight <= endRight {
				idxRight := (startRight + endRight) / 2
				if nums[idxRight] == target {
					startRight = idxRight + 1
				} else {
					// nums[idxRight] > target
					endRight = idxRight - 1
				}
			}
			idxRight := endRight

			return []int{idxLeft, idxRight}
		} else if nums[idx] < target {
			start = idx + 1
		} else {
			end = idx - 1
		}
	}

	return []int{-1, -1}
}
