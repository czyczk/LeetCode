package main

import "fmt"

func main() {
	arr1 := []int{1, 2, 1, 2, 3}
	k1 := 2

	arr2 := []int{1, 2, 1, 3, 4}
	k2 := 3

	// Expecting 7
	fmt.Println(subarraysWithKDistinct(arr1, k1))
	// Expecting 3
	fmt.Println(subarraysWithKDistinct(arr2, k2))
}

func subarraysWithKDistinct(A []int, K int) int {
	return getNumArraysWithAtMostKDistinct(A, K) - getNumArraysWithAtMostKDistinct(A, K-1)
}

func getNumArraysWithAtMostKDistinct(arr []int, k int) int {
	n := len(arr)
	freq := make([]int, n+1)

	left, right := 0, 0
	count := 0
	res := 0

	for right < n {
		if freq[arr[right]] == 0 {
			count++
		}

		freq[arr[right]]++
		right++

		for count > k {
			freq[arr[left]]--
			if freq[arr[left]] == 0 {
				count--
			}

			left++
		}

		res += right - left
	}

	return res
}
