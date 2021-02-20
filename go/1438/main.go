package main

import (
	"fmt"
	"sort"
)

func main() {
	nums1 := []int{8, 2, 4, 7}
	limit1 := 4

	nums2 := []int{10, 1, 2, 4, 7, 2}
	limit2 := 5

	nums3 := []int{4, 2, 2, 2, 4, 4, 2, 2}
	limit3 := 0

	nums4 := []int{1, 5, 6, 7, 8, 10, 6, 5, 6}
	limit4 := 4

	// Expecting 2
	fmt.Println(longestSubarray(nums1, limit1))
	// Expecting 4
	fmt.Println(longestSubarray(nums2, limit2))
	// Expecting 3
	fmt.Println(longestSubarray(nums3, limit3))
	// Expecting 5
	fmt.Println(longestSubarray(nums4, limit4))
}

func longestSubarray(nums []int, limit int) int {
	left, right := 0, 0
	maxHeap := NewMaxHeap()
	maxHeap.Push(0)
	minHeap := NewMaxHeap()
	minHeap.Push(-int(^uint(0) >> 1))

	n := len(nums)
	maxWindowSize := 1
	curDiff := 0
	for right < n {
		num := nums[right]
		isModified := false
		maxNum := maxHeap.Peek()
		minNum := -minHeap.Peek()

		if num > maxNum {
			isModified = true
			maxNum = num
		}
		if num < minNum {
			isModified = true
			minNum = num
		}

		//fmt.Printf("right: %v, left: %v, max: %v, min: %v, mws: %v, m: %v\n", right, left, maxNum, minNum, maxWindowSize, isModified)

		maxHeap.Push(num)
		minHeap.Push(-num)

		leftNum := nums[left]
		if isModified {
			diff := abs(num - leftNum)
			curDiff = diff
		}

		if curDiff > limit {
			// Shift `left` right
			maxHeap.Remove(leftNum)
			minHeap.Remove(-leftNum)
			if leftNum == maxNum {
				maxHeap.Pop()
			}
			if leftNum == minNum {
				minHeap.Pop()
			}
			left++
		} else {
			// Check if the current window size is larger
			maxWindowSize = max(maxWindowSize, right-left+1)
		}

		//fmt.Println("mws:", maxWindowSize)
		right++
	}

	return maxWindowSize
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func max(x, y int) int {
	if x > y {
		return x
	}
	return y
}

type MaxHeap struct {
	sort.IntSlice
	delayedRemoval map[int]int
}

func NewMaxHeap() *MaxHeap {
	return &MaxHeap{
		delayedRemoval: make(map[int]int),
	}
}

func (h *MaxHeap) Push(x interface{}) {
	h.IntSlice = append(h.IntSlice, x.(int))
}

func (h *MaxHeap) Pop() interface{} {
	slice := h.IntSlice
	ret := slice[len(slice)-1]
	h.IntSlice = slice[:len(slice)-1]

	// Perform delayed removals
	numTop := h.Peek()
	freq, ok := h.delayedRemoval[numTop]
	if ok {
		h.IntSlice = slice[:len(slice)-1-freq]
		delete(h.delayedRemoval, numTop)
	}

	return ret
}

func (h *MaxHeap) Peek() int {
	slice := h.IntSlice
	return slice[len(slice)-1]
}

func (h *MaxHeap) Remove(num int) {
	freq, ok := h.delayedRemoval[num]
	if ok {
		h.delayedRemoval[num] = freq + 1
	} else {
		h.delayedRemoval[num] = 1
	}
}
