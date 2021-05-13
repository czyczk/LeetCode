package main

import "fmt"

func main() {
	head := &ListNode{
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
	printLinkedList(reverseList(head))
}

func printLinkedList(head *ListNode) {
	cur := head
	fmt.Printf("[")
	isFirst := true

	for cur != nil {
		if !isFirst {
			fmt.Printf(", ")
		}
		fmt.Printf("%v", cur.Val)
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

func reverseList(head *ListNode) *ListNode {
	var prev *ListNode = nil
	cur := head

	for cur != nil {
		next := cur.Next
		cur.Next = prev
		prev = cur
		cur = next
	}

	return prev
}
