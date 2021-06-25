package main

func main() {
	g1, s1 := []int{1, 2, 3}, []int{1, 1}
	g2, s2 := []int{1, 2}, []int{1, 2, 3}

	// Expecting 1
	println(findContentChildren(g1, s1))
	// Expecting 2
	println(findContentChildren(g2, s2))
}

func findContentChildren(g []int, s []int) int {
	quickSortReverse(g)
	quickSortReverse(s)

	gIdx := 0
	sIdx := 0
	numSatisfied := 0

	for sIdx < len(s) {
		if s[sIdx] >= g[gIdx] {
			numSatisfied++
			sIdx++
		}

		gIdx++
		if gIdx == len(g) {
			break
		}
	}

	return numSatisfied
}

func quickSortReverse(arr []int) {
	quickSortReverseCore(arr, 0, len(arr)-1)
}

func quickSortReverseCore(arr []int, start, end int) {
	if start >= end {
		return
	}

	pivot := arr[start]
	i, j := start, end

	for i != j {
		// From right to left, stop where arr[j] >= pivot
		for j > i && arr[j] < pivot {
			j--
		}

		// From left to right, stop where arr[i] < pivot
		for i < j && arr[i] >= pivot {
			i++
		}

		if i < j {
			temp := arr[i]
			arr[i] = arr[j]
			arr[j] = temp
		}
	}

	arr[start] = arr[i]
	arr[i] = pivot

	quickSortReverseCore(arr, start, i-1)
	quickSortReverseCore(arr, i+1, end)
}
