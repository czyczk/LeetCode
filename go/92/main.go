package main

import "fmt"

func main() {
	head1 := &ListNode{
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

	head2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val:  4,
					Next: nil,
				},
			},
		},
	}

	head3 := &ListNode{
		Val:  5,
		Next: nil,
	}

	// Expecting [1, 4, 3, 2, 5]
	printLinkedList(reverseBetween(head1, 2, 4))
	// Expecting [3, 2, 1, 4]
	printLinkedList(reverseBetween(head2, 1, 3))
	// Expecting [5]
	printLinkedList(reverseBetween(head3, 1, 1))
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
 **/
type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseBetween(head *ListNode, left int, right int) *ListNode {
	var fakeHead = &ListNode{
		Val:  2333,
		Next: head,
	}

	nodeLeftPrev := fakeHead
	nodeLeft := head
	nodeRight := head
	for i := 1; i < right; i++ {
		if i < left {
			nodeLeftPrev = nodeLeft
			nodeLeft = nodeLeft.Next
		}
		nodeRight = nodeRight.Next
	}

	nodeRightSucc := nodeRight.Next

	var prev *ListNode = nil
	var cur = nodeLeft
	for cur != nodeRightSucc {
		next := cur.Next
		cur.Next = prev
		prev = cur
		cur = next
	}

	// Now `prev` is the first in the reversed area;
	// `nodeLeft` is the last in the reverse area.
	nodeLeftPrev.Next = prev
	nodeLeft.Next = nodeRightSucc

	return fakeHead.Next

}
