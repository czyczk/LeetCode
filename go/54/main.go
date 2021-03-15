package main

import "fmt"

func main() {
	m1 := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}

	m2 := [][]int{
		{1, 2, 3, 4},
		{5, 6, 7, 8},
		{9, 10, 11, 12},
	}

	m3 := [][]int{
		{1, 2, 3, 4},
		{5, 6, 7, 8},
		{9, 10, 11, 12},
		{13, 14, 15, 16},
	}

	// Expecting [1, 2, 3, 6, 9, 8, 7, 4, 5]
	fmt.Printf("%v\n", spiralOrder(m1))
	// Expecting [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
	fmt.Printf("%v\n", spiralOrder(m2))
	// Expecting [1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
	fmt.Printf("%v\n", spiralOrder(m3))
}

type Direction int

const (
	right Direction = iota
	down
	left
	up
)

func (d Direction) getNextDirection() Direction {
	switch d {
	case right:
		return down
	case down:
		return left
	case left:
		return up
	case up:
		return right
	default:
		panic("Unknown direction")
	}
}

func (d Direction) getNextIdx(i, j int) (int, int) {
	switch d {
	case right:
		return i, j + 1
	case down:
		return i + 1, j
	case left:
		return i, j - 1
	case up:
		return i - 1, j
	default:
		panic("Unknown direction")
	}
}

func spiralOrder(matrix [][]int) []int {
	n := len(matrix)
	m := len(matrix[0])
	// 0: right, 1: down, 2: left, 3: up
	totalSteps := []int{m, n - 1, m - 1, n - 2}
	remainingSteps := make([]int, 4)
	copy(remainingSteps, totalSteps)

	cnt, i, j := 1, 0, 0
	ret := make([]int, m*n)
	ret[0] = matrix[i][j]
	curDirection := right
	remainingSteps[0]--

	for cnt < m*n {
		if remainingSteps[int(curDirection)] > 0 {
			remainingSteps[int(curDirection)]--
			i, j = curDirection.getNextIdx(i, j)
			ret[cnt] = matrix[i][j]
			cnt++
		} else {
			totalSteps[int(curDirection)] -= 2
			remainingSteps[int(curDirection)] = totalSteps[int(curDirection)]
			curDirection = curDirection.getNextDirection()
		}
	}

	return ret
}
