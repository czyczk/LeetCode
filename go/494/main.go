package main

func main() {
	n1, t1 := []int{1, 1, 1, 1, 1}, 3
	n2, t2 := []int{1}, 1
	n3, t3 := []int{1000}, -1000
	n4, t4 := []int{0, 0, 0, 0, 0, 0, 0, 0, 1}, 1

	// Expecting 5
	println(findTargetSumWays(n1, t1))
	// Expecting 1
	println(findTargetSumWays(n2, t2))
	// Expecting 1
	println(findTargetSumWays(n3, t3))
	// Expecting 256
	println(findTargetSumWays(n4, t4))
}

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}

	if target > sum || target < -sum {
		return 0
	}

	if (target+sum)%2 != 0 {
		return 0
	}

	target = (target + sum) / 2

	dp := make([]int, target+1)
	dp[0] = 1

	for i := 0; i < len(nums); i++ {
		for j := target; j >= nums[i]; j-- {
			dp[j] += dp[j-nums[i]]
		}
	}

	return dp[target]
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
