package main

import "fmt"

func main() {
	// Expecting [1, 3, 3, 1]
	fmt.Printf("%v\n", getRow(3))
	// Expecting [1]
	fmt.Printf("%v\n", getRow(0))
	// Expecting [1, 1]
	fmt.Printf("%v\n", getRow(1))
}

func getRow(rowIndex int) []int {
	row := make([]int, rowIndex+1)
	row[0] = 1
	for i := 1; i <= rowIndex; i++ {
		row[i] = row[i-1] * (rowIndex - i + 1) / i
	}

	return row
}
