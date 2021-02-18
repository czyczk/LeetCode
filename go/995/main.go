package main

import "fmt"

func main() {
	arr1 := []int{0, 1, 0}
	k1 := 1
	arr2 := []int{1, 1, 0}
	k2 := 2
	arr3 := []int{0, 0, 0, 1, 0, 1, 1, 0}
	k3 := 3

	// Expecting 2
	fmt.Println(minKBitFlips(arr1, k1))
	// Expecting -1
	fmt.Println(minKBitFlips(arr2, k2))
	// Expecting 3
	fmt.Println(minKBitFlips(arr3, k3))
}

func minKBitFlips(arr []int, k int) int {
	n := len(arr)
	flipArr := make([]int, n+1)
	flipCnt := 0
	actionCnt := 0

	for i, num := range arr {
		// revCnt += flipArr[i]
		flipCnt ^= flipArr[i]

		// Check if num needs to be flipped
		// (num + revCnt) % 2 == 0 =>
		// (num ^ revCnt) == 0 =>
		if num == flipCnt {
			if i+k > n {
				return -1
			}

			// flipArr[i + k] += 1
			flipArr[i+k] ^= 1
			// revCnt += 1
			flipCnt ^= 1

			actionCnt++
		}
	}

	return actionCnt
}
