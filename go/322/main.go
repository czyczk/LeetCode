package main

func main() {
	c1, a1 := []int{1, 2, 5}, 11
	c2, a2 := []int{2}, 3
	c3, a3 := []int{1}, 0
	c4, a4 := []int{1}, 1
	c5, a5 := []int{1}, 2

	// Expecting 3
	println(coinChange(c1, a1))
	println(coinChangeRecursiveSolution(c1, a1))
	// Expecting -1
	println(coinChange(c2, a2))
	println(coinChangeRecursiveSolution(c2, a2))
	// Expecting 0
	println(coinChange(c3, a3))
	println(coinChangeRecursiveSolution(c3, a3))
	// Expecting 1
	println(coinChange(c4, a4))
	println(coinChangeRecursiveSolution(c4, a4))
	// Expecting 2
	println(coinChange(c5, a5))
	println(coinChangeRecursiveSolution(c5, a5))
}

func coinChange(coins []int, amount int) int {
	m := len(coins)
	n := amount
	dp := make([]int, n+1)
	for j := 1; j <= n; j++ {
		dp[j] = -1
	}

	for i := 0; i < m; i++ {
		for j := coins[i]; j <= n; j++ {
			if dp[j-coins[i]] == -1 {
				continue
			}

			if dp[j] == -1 {
				dp[j] = dp[j-coins[i]] + 1
			} else {
				dp[j] = min(dp[j], dp[j-coins[i]]+1)
			}
		}
	}

	return dp[n]
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
