package main

import (
	"fmt"
)

func main() {
	cn1 := 1
	cn2 := 28
	cn3 := 701
	cn4 := 2147483647

	// Expecting "A"
	fmt.Printf("%v\n", convertToTitle(cn1))
	// Expecting "AB"
	fmt.Printf("%v\n", convertToTitle(cn2))
	// Expecting "ZY"
	fmt.Printf("%v\n", convertToTitle(cn3))
	// Expecting "FXSHRXW"
	fmt.Printf("%v\n", convertToTitle(cn4))
}

func convertToTitle(columnNumber int) string {
	var b []byte
	for columnNumber != 0 {
		columnNumber--
		rem := columnNumber % 26

		b = append(b, byte(rem)+'A')

		columnNumber -= rem
		columnNumber /= 26
	}

	reverse(b)
	return string(b)
}

func reverse(b []byte) {
	for i, j := 0, len(b)-1; i < j; i, j = i+1, j-1 {
		b[i], b[j] = b[j], b[i]
	}
}
