package main

import (
	"fmt"
)

func main() {
	A1 := []int{1, 2, 0, 0}
	K1 := 34
	A2 := []int{2, 7, 4}
	K2 := 181
	A3 := []int{2, 1, 5}
	K3 := 806
	A4 := []int{9, 9, 9, 9, 9, 9, 9, 9, 9, 9}
	K4 := 1

	// Expecting [1, 2, 3, 4]
	fmt.Println(addToArrayForm(A1, K1))
	// Expecting [4, 5, 5]
	fmt.Println(addToArrayForm(A2, K2))
	// Expecting [1, 0, 2, 1]
	fmt.Println(addToArrayForm(A3, K3))
	// Expecting [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
	fmt.Println(addToArrayForm(A4, K4))
}

func addToArrayForm(A []int, K int) []int {
	BStr := fmt.Sprintf("%v", K)
	B := make([]int, len(BStr))
	for i, char := range []byte(BStr) {
		B[i] = int(char - byte('0'))
	}

	if len(B) > len(A) {
		A, B = B, A
	}

	longLen := len(A)
	shortLen := len(B)

	incr := 0
	for i := 1; i <= longLen; i++ {
		digit := A[longLen-i] + incr
		if i <= shortLen {
			digit += B[shortLen-i]
		}
		incr = 0

		if digit >= 10 {
			incr = 1
			digit -= 10
		}

		A[longLen-i] = digit
	}

	if incr == 1 {
		A = append([]int{1}, A...)
	}

	return A
}
