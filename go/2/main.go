package main

import "fmt"

func main() {
	l11, l21 := &ListNode{
		Val: 2,
		Next: &ListNode{
			Val: 4,
			Next: &ListNode{
				Val:  3,
				Next: nil,
			},
		},
	}, &ListNode{
		Val: 5,
		Next: &ListNode{
			Val: 6,
			Next: &ListNode{
				Val:  4,
				Next: nil,
			},
		},
	}

	l12, l22 := &ListNode{
		Val:  0,
		Next: nil,
	}, &ListNode{
		Val:  0,
		Next: nil,
	}

	l13, l23 := &ListNode{
		Val: 9,
		Next: &ListNode{
			Val: 9,
			Next: &ListNode{
				Val: 9,
				Next: &ListNode{
					Val: 9,
					Next: &ListNode{
						Val: 9,
						Next: &ListNode{
							Val: 9,
							Next: &ListNode{
								Val:  9,
								Next: nil,
							},
						},
					},
				},
			},
		},
	}, &ListNode{
		Val: 9,
		Next: &ListNode{
			Val: 9,
			Next: &ListNode{
				Val: 9,
				Next: &ListNode{
					Val:  9,
					Next: nil,
				},
			},
		},
	}

	// Expecting [7, 0, 8]
	printLinkedList(addTwoNumbers(l11, l21))
	// Expecting [0]
	printLinkedList(addTwoNumbers(l12, l22))
	// Expecting [8, 9, 9, 9, 0, 0, 0, 1]
	printLinkedList(addTwoNumbers(l13, l23))
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

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	ansHead := &ListNode{
		Val:  0,
		Next: nil,
	}
	ansCur := ansHead
	incr := false

	for l1 != nil || l2 != nil || incr {
		digit := 0
		if l1 != nil {
			digit += l1.Val
			l1 = l1.Next
		}

		if l2 != nil {
			digit += l2.Val
			l2 = l2.Next
		}

		if incr {
			digit++
			incr = false
		}

		if digit >= 10 {
			digit -= 10
			incr = true
		}

		newNode := &ListNode{
			Val:  digit,
			Next: nil,
		}
		ansCur.Next = newNode
		ansCur = newNode
	}

	return ansHead.Next
}
