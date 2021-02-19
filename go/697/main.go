package main

import (
	"fmt"
)

func main() {
	nums1 := []int{1, 2, 2, 3, 1}
	nums2 := []int{1, 2, 2, 3, 1, 4, 2}

	// Expecting 2
	fmt.Println(findShortestSubArray(nums1))
	// Expecting 6
	fmt.Println(findShortestSubArray(nums2))
}

func findShortestSubArray(nums []int) int {
	freqMap := make(map[int]int)
	leftMap := make(map[int]int)

	maxFreq := 0
	minRange := len(nums)

	for i, num := range nums {
		freq, ok := freqMap[num]
		if ok {
			freqMap[num]++
			freq++
		} else {
			freqMap[num] = 1
			freq = 1
		}

		_, ok = leftMap[num]
		if !ok {
			leftMap[num] = i
		}

		if freq < maxFreq {
			continue
		} else if freq > maxFreq {
			maxFreq = freq
			minRange = len(nums)
		}

		left := leftMap[num]
		candidateRange := i - left + 1
		if candidateRange < minRange {
			minRange = candidateRange
		}
	}

	return minRange
}
