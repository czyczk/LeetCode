package main

func main() {
	n1 := []int{-1, 0, 3, 5, 9, 12}
	t1 := 9

	n2 := []int{-1, 0, 3, 5, 9, 12}
	t2 := 2

	// Expecting 4
	println(search(n1, t1))

	// Expecting -1
	println(search(n2, t2))
}

func search(nums []int, target int) int {
	left, right := 0, len(nums)-1
	for left <= right {
		mid := (left + right) / 2
		if nums[mid] == target {
			return mid
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return -1
}
