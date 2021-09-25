package main

func main() {
	w11, w12 := "sea", "eat"
	w21, w22 := "leetcode", "etco"

	// Expecting 2
	println(minDistance(w11, w12))
	// Expecting 4
	println(minDistance(w21, w22))

}

func minDistance(word1 string, word2 string) int {
	m := len(word1)
	n := len(word2)
	dp := make([][]int, m+1)
	for i := 0; i <= m; i++ {
		dp[i] = make([]int, n+1)
		dp[i][0] = i
	}

	for j := 0; j <= n; j++ {
		dp[0][j] = j
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if word1[i-1] == word2[j-1] {
				dp[i][j] = dp[i-1][j-1]
			} else {
				dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + 1
			}
		}
	}

	return dp[m][n]
}

func min(x, y int) int {
	if x < y {
		return x
	}

	return y
}
