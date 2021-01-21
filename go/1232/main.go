package main

import "fmt"

func main() {
	c1 := [][]int{{1, 2}, {2, 3}, {3, 4}, {4, 5}, {5, 6}, {6, 7}}
	c2 := [][]int{{1, 1}, {2, 2}, {3, 4}, {4, 5}, {5, 6}, {7, 7}}
	c3 := [][]int{{0, 0}, {0, 1}, {0, -1}}

	// Expecting true
	fmt.Println(checkStraightLine(c1))
	// Expecting false
	fmt.Println(checkStraightLine(c2))
	// Expecting true
	fmt.Println(checkStraightLine(c3))
}

func checkStraightLine(coordinates [][]int) bool {
	x0 := coordinates[0][0]
	y0 := coordinates[0][1]
	aBase := coordinates[1][0] - x0
	bBase := coordinates[1][1] - y0

	n := len(coordinates)
	for i := 2; i < n; i++ {
		ai := coordinates[i][0] - x0
		bi := coordinates[i][1] - y0
		if aBase*bi-bBase*ai != 0 {
			return false
		}
	}

	return true
}
