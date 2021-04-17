package main

import "fmt"

func main() {
	m1, n1, k1 := 2, 3, 1
	m2, n2, k2 := 3, 1, 0
	m3, n3, k3 := 4, 11, 13
	m4, n4, k4 := 38, 15, 9
	m5, n5, k5 := 16, 8, 4

	// Expecting 3
	fmt.Println(movingCount(m1, n1, k1))

	// Expecting 1
	fmt.Println(movingCount(m2, n2, k2))

	// Expecting 1
	fmt.Println(movingCount(m3, n3, k3))

	// Expecting 135
	fmt.Println(movingCount(m4, n4, k4))

	// Expecting 15
	fmt.Println(movingCount(m5, n5, k5))
}

type Status int

const (
	Available Status = iota
	Prohibited
)

func movingCount(m, n, k int) int {
	if k == 0 {
		return 1
	}

	board := make([][]Status, m)
	for i := 0; i < m; i++ {
		numI := i
		sumI := 0
		x := 10
		for {
			rem := numI % x
			sumI += rem * 10 / x
			numI -= rem
			if numI == 0 {
				break
			}
			x *= 10
		}
		board[i] = make([]Status, n)
		for j := 0; j < n; j++ {
			numJ := j
			sum := sumI
			x := 10
			for sum <= k {
				rem := numJ % x
				sum += rem * 10 / x
				numJ -= rem
				if numJ == 0 {
					break
				}
				x *= 10
			}

			if sum > k {
				board[i][j] = Prohibited
			}
		}
	}

	maxSteps := 0

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			isConnected := board[i][j] == Available

			if isConnected && (i != 0 || j != 0) {
				isLeftConnected := j-1 >= 0 && board[i][j-1] == Available
				isUpConnected := i-1 >= 0 && board[i-1][j] == Available
				if !isLeftConnected && !isUpConnected {
					isConnected = false
					board[i][j] = Prohibited
				}
			}

			if isConnected {
				maxSteps++
			}
		}
	}

	return maxSteps
}
