package main

import (
	"fmt"
	"math/rand"
)

func main() {
	n1 := []int{-1, 0, 1, 2, -1, -4}
	n2 := []int{}
	n3 := []int{0}

	// Expecting [[-1, -1, 2], [-1, 0, 1]]
	fmt.Printf("%v\n", threeSum(n1))
	// Expecting []
	fmt.Printf("%v\n", threeSum(n2))
	// Expecting []
	fmt.Printf("%v\n", threeSum(n3))
}

func threeSum(nums []int) [][]int {
	n := len(nums)
	if n < 3 {
		return nil
	}

	quickSort(nums)
	minComplement := nums[0] + nums[1]
	maxComplement := nums[n-1] + nums[n-2]

	var ret [][]int

	lastNum := 10001
	for i, num := range nums {
		if num == lastNum {
			continue
		}

		targetComplement := -num
		if (num < 0 && maxComplement < targetComplement) || (num > 0 && minComplement > targetComplement) {
			continue
		}

		j := i + 1
		k := n - 1

		for j < k {
			numJ, numK := nums[j], nums[k]
			complement := numJ + numK
			if complement <= targetComplement {
				if complement == targetComplement {
					ret = append(ret, []int{num, numJ, numK})
				}
				// Move j right until nums[j] != numJ. Also i cannot be used again.
				for j < n && (i == j || nums[j] == numJ) {
					j++
				}
			} else {
				// Move k left until nums[k] != numK. Also i cannot be used again.
				for k >= 0 && (i == j || nums[k] == numK) {
					k--
				}
			}
		}

		lastNum = num
	}

	return ret
}

func quickSort(nums []int) {
	quickSortCore(nums, 0, len(nums)-1)
}

func quickSortCore(nums []int, start, end int) {
	if start >= end {
		return
	}

	idxPivot := rand.Intn(end-start+1) + start
	swap(nums, start, idxPivot)
	pivot := nums[start]

	// Both inclusive
	i, j := start, end
	for i < j {
		// j: Stop where nums[j] <= pivot
		for i < j && nums[j] > pivot {
			j--
		}

		// i: Stop where nums[i] > pivot
		for i < j && nums[i] <= pivot {
			i++
		}

		if i < j {
			swap(nums, i, j)
		}
	}

	swap(nums, start, i)

	quickSortCore(nums, start, i-1)
	quickSortCore(nums, i+1, end)
}

func swap(nums []int, i, j int) {
	temp := nums[i]
	nums[i] = nums[j]
	nums[j] = temp
}
