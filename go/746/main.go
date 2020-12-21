package main

import (
	"fmt"
)

func main() {
	cost1 := []int{10, 15, 20}
	cost2 := []int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1}
	cost3 := []int{0, 0, 0, 0}

	// Expecting 15
	fmt.Println(minCostClimbingStairs(cost1))
	// Expecting 6
	fmt.Println(minCostClimbingStairs(cost2))
	// Expecting 0
	fmt.Println(minCostClimbingStairs(cost3))
}

func minCostClimbingStairsOriginal(cost []int) int {
	n := len(cost)
	stepOnePrev := cost[0]
	stepOne := cost[0] + cost[1]
	stepTwoPrev := cost[0]
	stepTwo := cost[1]

	for i := 2; i < n; i++ {
		oldStepOne := stepOne
		oldStepTwo := stepTwo
		stepOne = min(stepOne, stepTwo) + cost[i]
		stepTwo = min(stepOnePrev, stepTwoPrev) + cost[i]
		stepOnePrev = oldStepOne
		stepTwoPrev = oldStepTwo
	}

	return min(min(min(stepOne, stepTwo), stepOnePrev), stepTwoPrev)
}

func minCostClimbingStairs(cost []int) int {
	n := len(cost)
	costPrev := cost[0]
	costCur := cost[1]

	for i := 2; i < n; i++ {
		newCostCur := min(costCur, costPrev) + cost[i]
		costPrev = costCur
		costCur = newCostCur
	}

	return min(costPrev, costCur)
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}
