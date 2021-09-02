package main

func main() {
	h1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val: 3,
					Next: &ListNode{
						Val: 4,
						Next: &ListNode{
							Val: 4,
							Next: &ListNode{
								Val:  5,
								Next: nil,
							},
						},
					},
				},
			},
		},
	}

	h2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val: 2,
					Next: &ListNode{
						Val:  3,
						Next: nil,
					},
				},
			},
		},
	}

	// Expecting [1, 2, 5]
	printLinkedList(deleteDuplicates(h1))

	// Expecting [2, 3]
	printLinkedList(deleteDuplicates(h2))
}

func printLinkedList(head *ListNode) {
	print("[")
	isFirst := true
	for head != nil {
		if !isFirst {
			print(", ")
		}

		print(head.Val)
		head = head.Next
		isFirst = false
	}
	println("]")
}

/**
 * Definition for singly-linked list.
 */
type ListNode struct {
	Val  int
	Next *ListNode
}

func deleteDuplicates(head *ListNode) *ListNode {
	fakeHead := &ListNode{
		Val:  0,
		Next: head,
	}

	cur := fakeHead

	for cur.Next != nil {
		nextNum := cur.Next.Val
		isDuplicate := false
		for cur.Next.Next != nil && cur.Next.Next.Val == nextNum {
			cur.Next.Next = cur.Next.Next.Next
			isDuplicate = true
		}

		if isDuplicate {
			cur.Next = cur.Next.Next
		} else {
			cur = cur.Next
		}

	}

	return fakeHead.Next
}
