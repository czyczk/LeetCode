package main

import "fmt"

func main() {
	head1 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 9,
				},
			},
		},
	}

	deleteNode(head1.Next)
	checkAns([]int{4, 1, 9}, head1)

	head2 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 9,
				},
			},
		},
	}

	deleteNode(head2.Next.Next)
	checkAns([]int{4, 5, 9}, head2)

	head3 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 4,
				},
			},
		},
	}

	deleteNode(head3.Next.Next)
	checkAns([]int{1, 2, 4}, head3)

	head4 := &ListNode{
		Val: 0,
		Next: &ListNode{
			Val: 1,
		},
	}

	deleteNode(head4)
	checkAns([]int{1}, head4)

	head5 := &ListNode{
		Val: -3,
		Next: &ListNode{
			Val: 5,
			Next: &ListNode{
				Val: -99,
			},
		},
	}

	deleteNode(head5)
	checkAns([]int{5, -99}, head5)
}

func checkAns(expected []int, head *ListNode) {
	cur := head
	i := 0
	for cur != nil {
		if expected[i] != cur.Val {
			fmt.Printf("Not matched. Expected %v, found %v.\n", expected[i], cur.Val)
			return
		}

		cur = cur.Next
		i++
	}

	if i != len(expected) {
		fmt.Printf("Expected not consumed. Index: %v.\n", i)
		return
	}
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteNode(node *ListNode) {
	node.Val = node.Next.Val
	node.Next = node.Next.Next
}
