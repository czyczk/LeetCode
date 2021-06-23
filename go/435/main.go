package main

import (
	"fmt"
	"sort"
)

func main() {
	i1 := [][]int{{1, 2}, {2, 3}, {3, 4}, {1, 3}}
	i2 := [][]int{{1, 2}, {1, 2}, {1, 2}}
	i3 := [][]int{{1, 2}, {2, 3}}
	i4 := [][]int{{0, 1}, {3, 4}, {1, 2}}

	// Expecting 1
	fmt.Println(eraseOverlapIntervals(i1))
	// Expecting 2
	fmt.Println(eraseOverlapIntervals(i2))
	// Expecting 0
	fmt.Println(eraseOverlapIntervals(i3))
	// Expecting 0
	fmt.Println(eraseOverlapIntervals(i4))
}

func eraseOverlapIntervals(intervals [][]int) int {
	if len(intervals) == 0 {
		return 0
	}

	// Sort the intervals by their right end
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})

	numDropped := 0

	maxRightEnd := intervals[0][0]
	for _, interval := range intervals {
		if interval[1] > maxRightEnd && interval[0] >= maxRightEnd {
			maxRightEnd = interval[1]
		} else {
			numDropped++
		}
	}

	return numDropped
}
