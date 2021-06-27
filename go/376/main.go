package main

func main() {
	n1 := []int{1, 7, 4, 9, 2, 5}
	n2 := []int{1, 17, 5, 10, 13, 15, 10, 5, 16, 8}
	n3 := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
	n4 := []int{1}
	n5 := []int{1, 0}

	// Expecting 6
	println(wiggleMaxLength(n1))
	// Expecting 7
	println(wiggleMaxLength(n2))
	// Expecting 2
	println(wiggleMaxLength(n3))
	// Expecting 1
	println(wiggleMaxLength(n4))
	// Expecting 2
	println(wiggleMaxLength(n5))
}

func wiggleMaxLength(nums []int) int {
	n := len(nums)
	if n == 1 {
		return 1
	} else if n == 2 {
		if nums[0] != nums[1] {
			return 2
		}
		return 1
	}

	// fUp, fDown := make([]int, n), make([]int, n)
	// fUp[0] = 1
	// fDown[0] = 1
	fUp, fDown := 1, 1

	for i := 1; i < n; i++ {
		if nums[i] == nums[i-1] {
			// fUp[i] = fUp[i-1]
			// fDown[i] = fDown[i-1]
		} else if nums[i] < nums[i-1] {
			// fUp[i] = fUp[i-1]
			// fDown[i] = fUp[i-1] + 1
			fDown = fUp + 1
		} else {
			// fUp[i] = fDown[i-1] + 1
			// fDown[i] = fDown[i-1]
			fUp = fDown + 1
		}
	}

	// return max(fUp[n-1], fDown[n-1])
	return max(fUp, fDown)
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
