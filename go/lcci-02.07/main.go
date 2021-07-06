package main

func main() {
	intersectionNode1 := &ListNode{
		Val: 8,
		Next: &ListNode{
			Val: 4,
			Next: &ListNode{
				Val:  5,
				Next: nil,
			},
		},
	}
	listA1 := &ListNode{
		Val: 4,
		Next: &ListNode{
			Val:  1,
			Next: intersectionNode1,
		},
	}
	listB1 := &ListNode{
		Val: 5,
		Next: &ListNode{
			Val: 0,
			Next: &ListNode{
				Val:  1,
				Next: intersectionNode1,
			},
		},
	}

	intersectionNode2 := &ListNode{
		Val: 2,
		Next: &ListNode{
			Val:  4,
			Next: nil,
		},
	}
	listA2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val:  9,
			Next: intersectionNode2,
		},
	}
	listB2 := &ListNode{
		Val:  3,
		Next: intersectionNode2,
	}

	listA3 := &ListNode{
		Val: 2,
		Next: &ListNode{
			Val: 6,
			Next: &ListNode{
				Val:  4,
				Next: nil,
			},
		},
	}
	listB3 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val:  5,
			Next: nil,
		},
	}

	// Expecting ALL true
	println(getIntersectionNode(listA1, listB1) == intersectionNode1)
	println(getIntersectionNode(listA2, listB2) == intersectionNode2)
	println(getIntersectionNode(listA3, listB3) == nil)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	m, n := 0, 0
	curA, curB := headA, headB

	for curA != nil || curB != nil {
		if curA == curB {
			return curA
		}

		if curA != nil {
			m++
			curA = curA.Next
		}
		if curB != nil {
			n++
			curB = curB.Next
		}
	}

	curA = headA
	curB = headB

	if m == n {
		return nil
	} else if n > m {
		for i := 0; i < (n - m); i++ {
			curB = curB.Next
		}
	} else {
		for i := 0; i < (m - n); i++ {
			curA = curA.Next
		}
	}

	for curA != nil {
		if curA == curB {
			return curA
		}

		curA = curA.Next
		curB = curB.Next
	}

	return nil
}
