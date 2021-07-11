package main

func main() {
	// Expecting 5
	println(numTrees(3))

	// Expecting 1
	println(numTrees(1))

	// Expecting 2
	println(numTrees(2))

	// Expecting 42
	println(numTrees(5))
}

func numTrees(n int) int {
	dp := make([]int, n+1)

	dp[0] = 1
	dp[1] = 1
	for i := 2; i <= n; i++ {
		half := i / 2
		numBSTs := 0
		for j := 1; j <= half; j++ {
			numBSTs += dp[j-1] * dp[i-j] * 2
		}

		if i%2 == 1 {
			numBSTs += dp[half] * dp[half]
		}

		dp[i] = numBSTs
	}

	return dp[n]
}
