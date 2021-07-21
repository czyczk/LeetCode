package main

func main() {
	n1, f1, s1 := []int{0, 6, 5, 2, 2, 5, 1, 9, 4}, 1, 2
	n2, f2, s2 := []int{3, 8, 1, 3, 2, 1, 8, 9, 0}, 3, 2
	n3, f3, s3 := []int{2, 1, 5, 6, 0, 9, 5, 0, 3, 8}, 4, 3

	// Expecting 20
	println(maxSumTwoNoOverlap(n1, f1, s1))
	// Expecting 29
	println(maxSumTwoNoOverlap(n2, f2, s2))
	// Expecting 31
	println(maxSumTwoNoOverlap(n3, f3, s3))
}

func maxSumTwoNoOverlap(nums []int, firstLen int, secondLen int) int {
	n := len(nums)
	var firstLeftSum, secondLeftSum, firstRightSum, secondRightSum int
	firstLeftSums, secondLeftSums, firstRightSums, secondRightSums := make([]int, n), make([]int, n), make([]int, n), make([]int, n)

	for i := 0; i < firstLen-1; i++ {
		firstRightSum += nums[i]
	}

	for i := 0; i < secondLen-1; i++ {
		secondRightSum += nums[i]
	}

	for i := 0; i < n; i++ {
		firstLeftStartIdx := i - firstLen + 1
		firstLeftSum += nums[i]
		if firstLeftStartIdx > 0 {
			firstLeftSum -= nums[firstLeftStartIdx-1]
		}
		firstLeftSums[i] = firstLeftSum

		secondLeftStartIdx := i - secondLen + 1
		secondLeftSum += nums[i]
		if secondLeftStartIdx > 0 {
			secondLeftSum -= nums[secondLeftStartIdx-1]
		}
		secondLeftSums[i] = secondLeftSum

		firstRightEndIdx := i + firstLen - 1 // inclusive
		if firstRightEndIdx < n {
			firstRightSum += nums[firstRightEndIdx]
			if i > 0 {
				firstRightSum -= nums[i-1]
			}
			firstRightSums[i] = firstRightSum
		}

		secondRightEndIdx := i + secondLen - 1 // inclusive
		if secondRightEndIdx < n {
			secondRightSum += nums[secondRightEndIdx]
			if i > 0 {
				secondRightSum -= nums[i-1]
			}
			secondRightSums[i] = secondRightSum
		}
	}

	for i := 1; i < n; i++ {
		firstLeftSums[i] = max(firstLeftSums[i], firstLeftSums[i-1])
		secondLeftSums[i] = max(secondLeftSums[i], secondLeftSums[i-1])
	}

	for i := n - 2; i >= 0; i-- {
		firstRightSums[i] = max(firstRightSums[i], firstRightSums[i+1])
		secondRightSums[i] = max(secondRightSums[i], secondRightSums[i+1])
	}

	ret := 0
	for i := 1; i < n; i++ {
		ret = max(ret, firstLeftSums[i-1]+secondRightSums[i])
		ret = max(ret, secondLeftSums[i-1]+firstRightSums[i])
	}

	return ret
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
