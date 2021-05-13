package main

import "fmt"

func main() {
	// Expecting 18
	fmt.Println(integerBreak(8))

	// Exepcting 1
	fmt.Println(integerBreak(2))

	// Expecting 36
	fmt.Println(integerBreak(10))

	// Expecting 6
	fmt.Println(integerBreak(5))

	// Expecting 18
	fmt.Println(integerBreak(8))

	// Expecting 4
	fmt.Println(integerBreak(4))
}

func integerBreak(n int) int {
	predefined := []int{1, 2}
	if n < 4 {
		return predefined[n-2]
	}

	numThrees := n / 3
	rem := n % 3

	if rem == 0 {
		return pow(3, numThrees)
	} else if rem == 1 {
		return pow(3, numThrees-1) * 4
	} else {
		return pow(3, numThrees) * 2
	}
}

func pow(x, n int) int {
	if n == 0 {
		return 1
	}

	ret := x
	for i := 1; i < n; i++ {
		ret *= x
	}

	return ret
}
