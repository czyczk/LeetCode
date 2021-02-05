package main

import "fmt"

func main() {
	s1 := "abcd"
	t1 := "bcdf"
	maxCost1 := 3

	s2 := "abcd"
	t2 := "cdef"
	maxCost2 := 3

	s3 := "abcd"
	t3 := "acde"
	maxCost3 := 0

	// Expecting 3
	fmt.Println(equalSubstring(s1, t1, maxCost1))
	// Expecting 1
	fmt.Println(equalSubstring(s2, t2, maxCost2))
	// Expecting 1
	fmt.Println(equalSubstring(s3, t3, maxCost3))
}

func equalSubstring(s string, t string, maxCost int) int {
	n := len(s)
	costs := make([]int, n)

	idxL := 0
	idxR := 0
	remainingCost := maxCost

	for idxR < n {
		costs[idxR] = abs(int(s[idxR]) - int(t[idxR]))

		remainingCost -= costs[idxR]
		if remainingCost < 0 {
			remainingCost += costs[idxL]
			idxL++
		}

		idxR++
	}

	return idxR - idxL
}

func abs(num int) int {
	if num < 0 {
		return -num
	}

	return num
}
