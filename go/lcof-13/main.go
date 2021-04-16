package main

import "fmt"

func main() {
	m1, n1, k1 := 2, 3, 1
	m2, n2, k2 := 3, 1, 0
	m3, n3, k3 := 4, 11, 13

	// Expecting 3
	fmt.Println(movingCount(m1, n1, k1))

	// Expecting 1
	fmt.Println(movingCount(m2, n2, k2))

	// Expecting 1
	fmt.Println(movingCount(m3, n3, k3))
}

type Status int

const (
	Pending Status = iota
	Visited
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
			}

			if sum > k {
				board[i][j] = Prohibited
			}
		}
	}

	steps := make([][]int, m)
	for i := 0; i < m; i++ {
		steps[i] = make([]int, n)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if j-1 >= 0 {
				steps[i][j] = steps[i][j-1]
			} else if i-1 >= 0 {
				steps[i][j] = steps[i-1][j] + 1
			} else {
				steps[i][j] = 0
			}

			if board[i][j] == Pending {
				steps[i][j]++
			}
		}
	}

	return steps[m-1][n-1]
}
