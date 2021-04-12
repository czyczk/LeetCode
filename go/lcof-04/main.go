package main

import "fmt"

func main() {
	matrix := [][]int{
		[]int{1, 4, 7, 11, 15},
		[]int{2, 5, 8, 12, 19},
		[]int{3, 6, 9, 16, 22},
		[]int{10, 13, 14, 17, 24},
		[]int{18, 21, 23, 26, 30},
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

	return binarySearch(matrix, 0, 0, n-1, m-1, target)
}

func binarySearch(matrix [][]int, startX, startY, endX, endY int, target int) bool {
	if endX < startX && endY < startY {
		return false
	}

	midX := (startX + endX) / 2
	midY := (startY + endY) / 2

	midNum := matrix[midX][midY]
	if midNum == target {
		return true
	} else if midNum < target {
		return binarySearch(matrix, midX+1, midY+1, endX, endY, target)
	} else {
		return binarySearch(matrix, 0, 0, midX-1, midY-1, target)
	}
}
