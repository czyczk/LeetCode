package main

import "fmt"

func main() {
	h1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 4,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	}

	h2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 4,
					Next: &ListNode{
						Val:  5,
						Next: nil,
					},
				},
			},
		},
	}

	// Expecting [2, 1, 4, 3, 5]
	printLinkedList(reverseKGroup(h1, 2))
	// Expecting [3, 2, 1, 4, 5]
	printLinkedList(reverseKGroup(h2, 3))
}

func printLinkedList(head *ListNode) {
	cur := head
	isFirst := true
	fmt.Printf("[")
	for cur != nil {
		if !isFirst {
			fmt.Printf(", ")
		}
		fmt.Printf("%v", cur.Val)
		isFirst = false
		cur = cur.Next
	}
	fmt.Println("]")
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseKGroup(head *ListNode, k int) *ListNode {
	fakeHead := &ListNode{
		Val:  0,
		Next: head,
	}
	cur := head
	var prev *ListNode
	willBeLast := head
	prevLast := fakeHead

	count := 0
	for cur != nil {
		oriNext := cur.Next
		if count == 0 {
			// Check if there's enough elements to form a group
			sentinel := cur
			sentinelCount := 0
			isEnough := false
			for sentinel != nil {
				sentinelCount++
				sentinel = sentinel.Next
				if sentinelCount == k {
					isEnough = true
					break
				}
			}
			if !isEnough {
				prevLast.Next = cur
				break
			}
			willBeLast = cur
		}

		cur.Next = prev

		count++

		if count == k {
			prevLast.Next = cur
			prevLast = willBeLast
			count = 0
			prev = nil
		} else {
			prev = cur
		}

		cur = oriNext
	}

	return fakeHead.Next
}
