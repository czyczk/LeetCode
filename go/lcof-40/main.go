package main

import "fmt"

func main() {
	a1 := []int{3, 2, 1}
	a2 := []int{0, 1, 2, 1}
	a3 := []int{0, 0, 1, 2, 4, 2, 2, 3, 1, 4}

	// Expecting [1, 2]
	fmt.Printf("%v\n", getLeastNumbers(a1, 2))

	// Expecting [0]
	fmt.Printf("%v\n", getLeastNumbers(a2, 1))

	// Expecting [0, 0, 1, 1, 2, 2, 2, 3]
	fmt.Printf("%v\n", getLeastNumbers(a3, 8))
}

func getLeastNumbers(arr []int, k int) []int {
	n := len(arr)
	if k >= n {
		return arr
	}

	l, r := 0, n-1
	for l < r {
		p := partition(arr, l, r)

		if p == k-1 {
			break
		} else if p < k-1 {
			l = p + 1
		} else {
			r = p - 1
		}
	}

	return arr[:k]
}

// [startIdx, endIdx]
func partition(arr []int, l, r int) int {
	if l >= r {
		return l
	}

	startIdx := l
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

	swap(arr, startIdx, l)
	return l
}

func swap(arr []int, x, y int) {
	arr[x], arr[y] = arr[y], arr[x]
}
