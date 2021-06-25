package main

func main() {
	n1 := []int{2, 3, 1, 1, 4}
	n2 := []int{2, 3, 0, 1, 4}
	n3 := []int{2, 1}
	n4 := []int{3, 2, 1}
	n5 := []int{0}

	// Expecting 2
	println(jump(n1))
	// Expecting 2
	println(jump(n2))
	// Expecting 1
	println(jump(n3))
	// Expecting 1
	println(jump(n4))
	// Expecting 0
	println(jump(n5))
}

func jump(nums []int) int {
	n := len(nums)
	if n == 1 {
		return 0
	}

	// Prevent index overflow
	coverage := min(nums[0], n-1)
	cur := 0
	steps := 0

	for {
		if coverage == n-1 {
			return steps + 1
		}

		maxCoverage := 0
		bestCandidate := cur + 1
		for i := cur + 1; i <= coverage; i++ {
			if nums[i]+i > maxCoverage {
				bestCandidate = i
				maxCoverage = nums[i] + i
			}
		}

		// Prevent index overflow
		coverage = min(maxCoverage, n-1)
		cur = bestCandidate

		steps++
	}
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
