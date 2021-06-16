package main

import "fmt"

func main() {
	amount1, coins1 := 5, []int{1, 2, 5}
	amount2, coins2 := 3, []int{2}
	amount3, coins3 := 10, []int{10}

	// Expecting 4
	fmt.Println(change(amount1, coins1))
	// Expecting 0
	fmt.Println(change(amount2, coins2))
	// Expecting 1
	fmt.Println(change(amount3, coins3))
}

func change(amount int, coins []int) int {
	f := make([]int, amount+1)
	f[0] = 1

	for _, c := range coins {
		for i := 1; i <= amount; i++ {
			if c > i {
				continue
			}

			f[i] += f[i-c]
		}
	}

	return f[amount]
}
