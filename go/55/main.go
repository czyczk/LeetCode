package main

func main() {
	n1 := []int{2, 3, 1, 1, 4}
	n2 := []int{3, 2, 1, 0, 4}

	// Expecting true
	println(canJump(n1))
	// Expecting false
	println(canJump(n2))
}

func canJump(nums []int) bool {
	coverage := 0
	i := 0

	for i <= coverage {
		coverage = max(coverage, i+nums[i])
		if coverage >= len(nums)-1 {
			return true
		}

		i++
	}

	return false
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
