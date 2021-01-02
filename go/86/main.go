package main

import "fmt"

func main() {
	list1 := &ListNode{Val: 1}
	head1 := list1
	list1.Next = &ListNode{Val: 4}
	list1 = list1.Next
	list1.Next = &ListNode{Val: 3}
	list1 = list1.Next
	list1.Next = &ListNode{Val: 2}
	list1 = list1.Next
	list1.Next = &ListNode{Val: 5}
	list1 = list1.Next
	list1.Next = &ListNode{Val: 2}
	x1 := 3

	// Expecting 1->2->2->4->3->5
	printLinkedList(partition(head1, x1))
}

// ListNode for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func partition(head *ListNode, x int) *ListNode {
	small := &ListNode{Val: 0}
	smallHead := small
	large := &ListNode{Val: 0}
	largeHead := large

	for head != nil {
		if head.Val < x {
			small.Next = head
			small = small.Next
		} else {
			large.Next = head
			large = large.Next
		}

		head = head.Next
	}

	large.Next = nil
	small.Next = largeHead.Next

	return smallHead.Next
}

func printLinkedList(head *ListNode) {
	if head == nil {
		fmt.Println("Nil")
		return
	}

	for head != nil {
		fmt.Print(head.Val)
		if head.Next != nil {
			fmt.Print(" -> ")
		}
		head = head.Next
	}

	fmt.Println()
}
