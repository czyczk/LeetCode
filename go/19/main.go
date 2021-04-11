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

	removeNthFromEnd(h1, 2)
	printLinkedList(h1)
}

func printLinkedList(head *ListNode) {
	cur := head
	fmt.Print("[")
	isFirst := true

	for cur != nil {
		if !isFirst {
			fmt.Print(", ")
		}
		fmt.Print(cur.Val)
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

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	fakeHead := ListNode{
		Val:  0,
		Next: head,
	}

	fast := &fakeHead
	for i := 0; i <= n; i++ {
		fast = fast.Next
	}
	slow := &fakeHead

	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}

	slow.Next = slow.Next.Next

	return fakeHead.Next
}
