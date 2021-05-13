package main

import "fmt"

func main() {
	// Expecting 1 to 9
	fmt.Printf("%v\n", printNumbers(1))
}

func printNumbers(n int) []int {
	limit := pow10(n)
	ret := make([]int, limit-1)

	for i := 1; i < limit; i++ {
		ret[i-1] = i
	}

	return ret
}

func pow10(n int) int {
	ret := 10
	for i := 1; i < n; i++ {
		ret *= 10
	}

	return ret
}
