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

	k1 := 2

	// Expecting [4, 5]
	printLinkedList(getKthFromEnd(h1, k1))
}

func printLinkedList(head *ListNode) {
	fmt.Print("[")
	isFirst := true
	cur := head
	for cur != nil {
		if !isFirst {
			fmt.Print(", ")
		}
		fmt.Print(cur.Val)
		cur = cur.Next
		isFirst = false
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

func getKthFromEnd(head *ListNode, k int) *ListNode {
	fast := head
	for i := 0; i < k; i++ {
		fast = fast.Next
	}

	slow := head
	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}

	return slow
}
