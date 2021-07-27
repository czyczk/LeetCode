package main

func main() {
	n1 := []int{1, 3, 5, 4, 7}
	n2 := []int{2, 2, 2, 2, 2}
	n3 := []int{1, 2, 4, 3, 5, 4, 7, 2}
	n4 := []int{1, 3, 2}

	// Expecting 2
	println(findNumberOfLIS(n1))
	// Expecting 5
	println(findNumberOfLIS(n2))
	// Expecting 3
	println(findNumberOfLIS(n3))
	// Expecting 2
	println(findNumberOfLIS(n4))
}

func findNumberOfLIS(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	numsReachingMax := make([]int, n)
	for i := 0; i < n; i++ {
		dp[i] = 1
		numsReachingMax[i] = 1
	}

	for i, num := range nums {
		for j := 0; j < i; j++ {
			if nums[j] >= num {
				continue
			}
			candidate := dp[j] + 1
			if candidate == dp[i] {
				numsReachingMax[i] += numsReachingMax[j]
			} else if candidate > dp[i] {
				dp[i] = candidate
				numsReachingMax[i] = numsReachingMax[j]
			}
		}
	}

	maxLen := dp[0]
	resCount := 0
	for i, l := range dp {
		if l == maxLen {
			resCount += numsReachingMax[i]
		} else if l > maxLen {
			maxLen = l
			resCount = numsReachingMax[i]
		}
	}

	return resCount
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
