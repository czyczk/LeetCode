package main

func main() {
	n1, t1 := []int{1, 2, 3}, 4
	n2, t2 := []int{9}, 3

	// Expecting 7
	println(combinationSum4(n1, t1))
	// Expecting 0
	println(combinationSum4(n2, t2))
}

func combinationSum4(nums []int, target int) int {
	dp := make([]int, target+1)
	dp[0] = 1

	for i := 1; i <= target; i++ {
		for _, num := range nums {
			if i < num {
				continue
			}
			dp[i] += dp[i-num]
		}
	}

	return dp[target]
}
