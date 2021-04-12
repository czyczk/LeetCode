package main

import "fmt"

func main() {
	matrix := [][]int{
		{1, 4, 7, 11, 15},
		{2, 5, 8, 12, 19},
		{3, 6, 9, 16, 22},
		{10, 13, 14, 17, 24},
		{18, 21, 23, 26, 30},
	}

	// Expecting true
	fmt.Println(findNumberIn2DArray(matrix, 5))
	// Expecting false
	fmt.Println(findNumberIn2DArray(matrix, 20))
}

func findNumberIn2DArray(matrix [][]int, target int) bool {
	n := len(matrix)
	if n == 0 {
		return false
	}
	m := len(matrix[0])
	if m == 0 {
		return false
	}

	i := 0
	j := m - 1

	for i < n && j >= 0 {
		num := matrix[i][j]
		if num == target {
			return true
		} else if num > target {
			j--
		} else {
			i++
		}
	}

	return false
}
