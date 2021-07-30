package main

import "fmt"

func main() {
	h1 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val:  3,
					Next: nil,
				},
			},
		},
	}

	h2 := &ListNode{
		Val: -1,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 4,
					Next: &ListNode{
						Val:  0,
						Next: nil,
					},
				},
			},
		},
	}

	h3 := (*ListNode)(nil)

	// Expecting [1, 2, 3, 4]
	printLinkedList(sortList(h1))
	// Expecting [-1, 0, 3, 4, 5]
	printLinkedList(sortList(h2))
	// Expecting []
	printLinkedList(sortList(h3))
}

func printLinkedList(head *ListNode) {
	isFirst := true
	fmt.Printf("[")
	cur := head
	for cur != nil {
		if !isFirst {
			fmt.Printf(", ")
		}
		fmt.Printf("%v", cur.Val)
		cur = cur.Next
		isFirst = false
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

func sortList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	fakeHead := &ListNode{
		Val:  0,
		Next: head,
	}
	subLen := 1

expansion:
	for {
		prev := fakeHead
		cur := fakeHead.Next
		isMerged := false

		for cur != nil {
			h1 := cur
			cur = skip(cur, subLen)
			h2 := cur
			if h2 == nil && !isMerged {
				break expansion
			}
			cur = skip(cur, subLen)
			mergeResult := mergeSorted(h1, h2, subLen, cur)
			isMerged = true
			prev.Next = mergeResult.Head
			prev = mergeResult.Tail
		}

		subLen <<= 1
	}

	return fakeHead.Next
}

func skip(cur *ListNode, steps int) *ListNode {
	for i := 0; i < steps && cur != nil; i++ {
		cur = cur.Next
	}

	return cur
}

func mergeSorted(h1, h2 *ListNode, length int, next *ListNode) (mergeResult MergeResult) {
	fakeHead := &ListNode{}
	cur := fakeHead
	idx1, idx2 := 0, 0

	for !isConsumed(h1, idx1, length) || !isConsumed(h2, idx2, length) {
		if isConsumed(h1, idx1, length) {
			cur.Next = h2
			h2 = h2.Next
			idx2++
		} else if isConsumed(h2, idx2, length) {
			cur.Next = h1
			h1 = h1.Next
			idx1++
		} else {
			if h1.Val < h2.Val {
				cur.Next = h1
				h1 = h1.Next
				idx1++
			} else {
				cur.Next = h2
				h2 = h2.Next
				idx2++
			}
		}

		cur = cur.Next
	}

	cur.Next = next

	mergeResult.Head = fakeHead.Next
	mergeResult.Tail = cur
	return
}

func isConsumed(cur *ListNode, i, length int) bool {
	return cur == nil || i >= length
}

type MergeResult struct {
	Head *ListNode
	Tail *ListNode
}
