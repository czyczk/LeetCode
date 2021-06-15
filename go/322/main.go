package main

import (
	"fmt"
)

func main() {
	coins1 := []int{1, 2, 5}
	amount1 := 11

	coins2 := []int{2}
	amount2 := 3

	coins3 := []int{1}
	amount3 := 0

	coins4 := []int{1}
	amount4 := 1

	coins5 := []int{1}
	amount5 := 2

	// Expecting 3 (11 = 5 + 5 + 1)
	fmt.Println(coinChange(coins1, amount1))
	// Expecting -1
	fmt.Println(coinChange(coins2, amount2))
	// Expecting 0
	fmt.Println(coinChange(coins3, amount3))
	// Expecting 1
	fmt.Println(coinChange(coins4, amount4))
	// Expecting 2
	fmt.Println(coinChange(coins5, amount5))
}

var f []int

func coinChange(coins []int, amount int) int {
	if amount == 0 {
		return 0
	}

	if len(coins) == 0 {
		return -1
	}

	f = make([]int, amount)

	return coinChangeRec(coins, amount)
}

func coinChangeRec(coins []int, rem int) int {
	if rem == 0 {
		return 0
	}

	minNum := f[rem-1]
	if minNum != 0 {
		return minNum
	}

	minNum = 10000
	for i := 0; i < len(coins); i++ {
		c := coins[i]
		if c > rem {
			continue
		}

		subRem := rem - c
		subMinNum := coinChangeRec(coins, subRem)
		if subMinNum != -1 && subMinNum < minNum {
			minNum = subMinNum
		}
	}

	if minNum == 10000 {
		f[rem-1] = -1
		return -1
	}

	f[rem-1] = minNum + 1
	return minNum + 1
}
