package main

import "fmt"

func main() {
	head := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 3,
			Next: &ListNode{
				Val:  2,
				Next: nil,
			},
		},
	}

	res := reversePrint(head)
	fmt.Printf("%v\n", res)
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func reversePrint(head *ListNode) []int {
	n := 0
	cur := head
	for cur != nil {
		n++
		cur = cur.Next
	}

	result := make([]int, n)

	cur = head
	for i := n - 1; i >= 0; i-- {
		result[i] = cur.Val
		cur = cur.Next
	}

	return result
}
