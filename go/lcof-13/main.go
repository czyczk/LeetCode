package main

import "fmt"

func main() {
	m1, n1, k1 := 2, 3, 1

	// Expecting 3
	fmt.Println(movingCount(m1, n1, k1))
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

	var stack [][]pair
	queue := []pair{{0, 0}}
	stack = append(stack, queue)
	var curTrace []pair
	maxSteps := 0

	for len(stack) != 0 {
		queue := stack[len(stack)-1]
		if len(queue) == 0 {
			if len(curTrace) == 0 {
				return maxSteps
			}

			idxRec := curTrace[len(curTrace)-1]
			board[idxRec.i][idxRec.j] = Pending
			curTrace = curTrace[:len(curTrace)-1]

			stack = stack[:len(stack)-1]

			continue
		}

		idx := queue[0]
		stack[len(stack)-1] = queue[1:]
		curTrace = append(curTrace, idx)
		maxSteps = max(maxSteps, len(curTrace))
		if maxSteps == m*n {
			return maxSteps
		}

		board[idx.i][idx.j] = Visited

		var adjCandidates []pair
		if idx.j-1 >= 0 && board[idx.i][idx.j-1] == Pending {
			adjCandidates = append(adjCandidates, pair{idx.i, idx.j - 1})
		}
		if idx.i-1 >= 0 && board[idx.i-1][idx.j] == Pending {
			adjCandidates = append(adjCandidates, pair{idx.i - 1, idx.j})
		}
		if idx.j+1 < n && board[idx.i][idx.j+1] == Pending {
			adjCandidates = append(adjCandidates, pair{idx.i, idx.j + 1})
		}
		if idx.i+1 < m && board[idx.i+1][idx.j] == Pending {
			adjCandidates = append(adjCandidates, pair{idx.i + 1, idx.j})
		}

		stack = append(stack, adjCandidates)
	}

	return maxSteps
}

type pair struct {
	i, j int
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}
