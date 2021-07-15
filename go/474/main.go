package main

func main() {
	s1, m1, n1 := []string{"10", "0001", "111001", "1", "0"}, 5, 3
	s2, m2, n2 := []string{"10", "0", "1"}, 1, 1

	// Expecting 4
	println(findMaxForm(s1, m1, n1))
	// Expecting 2
	println(findMaxForm(s2, m2, n2))
}

func findMaxForm(strs []string, m, n int) int {
	dp := make([][]int, m+1)
	for i := 0; i <= m; i++ {
		dp[i] = make([]int, n+1)
	}

	// dp init
	{
		str := strs[0]
		num0, num1 := 0, 0
		for i := 0; i < len(str); i++ {
			if str[i] == '0' {
				num0++
			} else {
				num1++
			}
		}

		for i := num0; i <= m; i++ {
			for j := num1; j <= n; j++ {
				dp[i][j] = 1
			}
		}
	}

	for i := 1; i < len(strs); i++ {
		str := strs[i]
		num0, num1 := 0, 0
		for i := 0; i < len(str); i++ {
			if str[i] == '0' {
				num0++
			} else {
				num1++
			}
		}

		for i := m; i >= num0; i-- {
			for j := n; j >= num1; j-- {
				dp[i][j] = max(dp[i][j], dp[i-num0][j-num1]+1)
			}
		}
	}

	return dp[m][n]
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
