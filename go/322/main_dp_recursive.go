package main

var f []int

func coinChangeRecursiveSolution(coins []int, amount int) int {
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
