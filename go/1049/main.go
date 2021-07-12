package main

func main() {
	s1 := []int{2, 7, 4, 1, 8, 1}
	s2 := []int{31, 26, 33, 21, 40}
	s3 := []int{1, 2}
	s4 := []int{91}

	// Expecting 1
	println(lastStoneWeightII(s1))
	// Expecting 5
	println(lastStoneWeightII(s2))
	// Expecting 1
	println(lastStoneWeightII(s3))
	// Expecting 91
	println(lastStoneWeightII(s4))
}

func lastStoneWeightII(stones []int) int {
	n := len(stones)
	if n == 1 {
		return stones[0]
	} else if n == 2 {
		return abs(stones[0] - stones[1])
	}

	sum := 0
	for _, stone := range stones {
		sum += stone
	}

	dp := make([]int, sum+1)
	for j := stones[0]; j <= sum; j++ {
		dp[j] = stones[0]
	}

	minDiff := 200
	for i := 1; i < n; i++ {
		for j := sum; j >= stones[i]; j-- {
			dp[j] = max(dp[j], dp[j-stones[i]]+stones[i])
			minDiff = min(minDiff, abs(dp[j]*2-sum))
		}
	}

	return minDiff
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
