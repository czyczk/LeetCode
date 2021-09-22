package main

func main() {
	h1, k1 := toLinkedList([]int{1, 2, 3}), 5
	h2, k2 := toLinkedList([]int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}), 3
	h3, k3 := toLinkedList([]int{0, 1, 2, 3, 4}), 3

	// Expecting [[1], [2], [3], [], []]
	printResult(splitListToParts(h1, k1))

	// Expecting [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
	printResult(splitListToParts(h2, k2))

	// Expecting [[0, 1], [2, 3], [4]]
	printResult(splitListToParts(h3, k3))
}

func toLinkedList(nums []int) *ListNode {
	fakeHead := &ListNode{}
	cur := fakeHead
	for _, num := range nums {
		cur.Next = &ListNode{
			Val: num,
		}
		cur = cur.Next
	}

	return fakeHead.Next
}

func printResult(ret []*ListNode) {
	print("[")

	for i, cur := range ret {
		if i != 0 {
			print(", ")
		}
		print("[")

		isFirst := true
		for cur != nil {
			if !isFirst {
				print(", ")
			}

			print(cur.Val)
			isFirst = false
			cur = cur.Next
		}

		print("]")
	}

	println("]")
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func splitListToParts(head *ListNode, k int) []*ListNode {
	n := 0
	cur := head
	for cur != nil {
		n++
		cur = cur.Next
	}

	var ret = make([]*ListNode, k)
	cur = head

	if k >= n {
		for i := 0; i < n; i++ {
			ret[i] = cur
			cur = cur.Next
			ret[i].Next = nil
		}

		return ret
	}

	avgGroupSize := n / k
	for i := 0; i < k; i++ {
		ret[i] = cur
		groupSize := avgGroupSize
		if i < n%k {
			groupSize++
		}

		for j := 0; j < groupSize; j++ {
			next := cur.Next
			if j == groupSize-1 {
				cur.Next = nil
			}
			cur = next
		}
	}

	return ret
}
