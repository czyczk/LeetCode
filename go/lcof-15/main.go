package main

import "fmt"

func main() {
	// Expecting 3
	fmt.Println(hammingWeight(0b1011))

	// Expecting 1
	fmt.Println(hammingWeight(0b10000000))

	// Expecting 31
	fmt.Println(hammingWeight(0b11111111111111111111111111111101))
}

func hammingWeight(num uint32) int {
	res := 0
	for num != 0 {
		res++
		num &= (num - 1)
	}

	return res
}
