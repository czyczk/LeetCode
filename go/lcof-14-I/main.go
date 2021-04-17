package main

import "fmt"

func main() {
	// Expecting 18
	fmt.Println(cuttingRope(8))

	// Exepcting 1
	fmt.Println(cuttingRope(2))

	// Expecting 36
	fmt.Println(cuttingRope(10))

	// Expecting 6
	fmt.Println(cuttingRope(5))

	// Expecting 18
	fmt.Println(cuttingRope(8))

	// Expecting 4
	fmt.Println(cuttingRope(4))
}

func cuttingRope(n int) int {
	preDefinedAns := []int{1, 2, 4}
	if n <= 3 {
		return preDefinedAns[n-2]
	}

	numCuts := n / 3
	rem := n % 3

	if rem == 0 {
		return pow(3, numCuts)
	} else if rem == 1 {
		return pow(3, numCuts-1) * 4
	} else {
		return pow(3, numCuts) * 2
	}
}

func pow(x, n int) int {
	res := 1
	for i := 0; i < n; i++ {
		res *= x
	}

	return res
}
