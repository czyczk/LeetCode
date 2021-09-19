package main

func main() {
	// Expecting 0
	println(minSteps(1))
	// Expecting 2
	println(minSteps(2))
	// Expecting 3
	println(minSteps(3))
	// Expecting 4
	println(minSteps(4))
	// Expecting 6
	println(minSteps(9))
	// Expecting 9
	println(minSteps(27))
}

var dp map[int]int

func minSteps(n int) int {
	dp = make(map[int]int)
	dp[1] = 0
	dp[2] = 2
	return getSteps(n)
}

func getSteps(n int) int {
	ret, ok := dp[n]
	if ok {
		return ret
	}

	if n%2 == 0 {
		ret = getSteps(n/2) + 2
		dp[n] = ret
		return ret
	} else {
		maxDiv := 3
		ok := false
		for i := 3; i <= n/2; i++ {
			if n%i == 0 {
				maxDiv = i
				ok = true
			}
		}

		if !ok {
			dp[n] = n
			return n
		}

		ret = getSteps(maxDiv) + n/maxDiv
		dp[n] = ret
		return ret
	}
}
