package main

import "fmt"

func main() {
	a1 := []int{1, 3, 5, 7, 2, 4, 6, 8}
	a2 := []int{6, 2, 7, 9, 3, 1, 1, 5}

	// The order doesn't matter
	// Expecting [1, 2, 3, 4]
	fmt.Printf("%v\n", smallestK(a1, 4))
	// Expecting [1, 1, 2, 3, 5]
	fmt.Printf("%v\n", smallestK(a2, 5))
}

func smallestK(arr []int, k int) []int {
	n := len(arr)
	if k >= n {
		return arr
	}

	l, r := 0, n-1
	for l < r {
		pos := partition(arr, l, r)
		if pos == k-1 {
			break
		} else if pos > k-1 {
			r = pos - 1
		} else {
			l = pos + 1
		}
	}

	return arr[:k]
}

func partition(arr []int, l, r int) int {
	start := l
	pivot := arr[l]

	for l < r {
		for l < r && arr[r] > pivot {
			r--
		}

		for l < r && arr[l] <= pivot {
			l++
		}

		if l < r {
			swap(arr, l, r)
		}
	}

	swap(arr, start, l)
	return l
}

func swap(arr []int, x, y int) {
	arr[x], arr[y] = arr[y], arr[x]
}
