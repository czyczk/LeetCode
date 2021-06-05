package main

import "fmt"

func main() {
	h1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 6,
				Next: &ListNode{
					Val: 3,
					Next: &ListNode{
						Val: 4,
						Next: &ListNode{
							Val: 5,
							Next: &ListNode{
								Val:  6,
								Next: nil,
							},
						},
					},
				},
			},
		},
	}

	h1 = removeElements(h1, 6)
	printLinkedList(h1)
}

func printLinkedList(head *ListNode) {
	isFirst := true
	cur := head
	for cur != nil {
		if !isFirst {
			fmt.Print(", ")
		}
		fmt.Print(cur.Val)
		isFirst = false
		cur = cur.Next
	}
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func removeElements(head *ListNode, val int) *ListNode {
	fakeHead := &ListNode{
		Val:  0,
		Next: head,
	}

	pre := fakeHead
	cur := head

	for cur != nil {
		if cur.Val == val {
			pre.Next = cur.Next
			cur = cur.Next
		} else {
			pre = cur
			cur = cur.Next
		}
	}

	return fakeHead.Next
}
