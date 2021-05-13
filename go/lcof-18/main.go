package main

import "fmt"

func main() {
	h1 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val:  9,
					Next: nil,
				},
			},
		},
	}

	h2 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val:  9,
					Next: nil,
				},
			},
		},
	}

	// Expecting [4, 1, 9]
	printLinkedList(deleteNode(h1, 5))

	// Expecting [4, 5, 9]
	printLinkedList(deleteNode(h2, 1))
}

func printLinkedList(head *ListNode) {
	fmt.Print("[")
	isFirst := true

	cur := head
	for cur != nil {
		if !isFirst {
			fmt.Print(", ")
		}

		fmt.Printf("%v", cur.Val)
		isFirst = false
		cur = cur.Next
	}

	fmt.Println("]")
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteNode(head *ListNode, val int) *ListNode {
	fakeHead := &ListNode{
		Val:  0,
		Next: head,
	}

	pre := fakeHead
	cur := head
	for cur != nil {
		if cur.Val == val {
			pre.Next = cur.Next
			break
		}

		pre = cur
		cur = cur.Next
	}

	return fakeHead.Next
}
