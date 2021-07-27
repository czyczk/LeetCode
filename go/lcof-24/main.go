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

	// Expecting [5, 4, 3, 2, 1]
	printLinkedList(reverseList(h1))
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

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	cur := head
	for cur != nil {
		oriNext := cur.Next
		cur.Next = prev
		prev = cur
		cur = oriNext
	}

	return prev
}
