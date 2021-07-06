package main

func main() {
	cycle1 := &ListNode{
		Val: 2,
		Next: &ListNode{
			Val: 0,
			Next: &ListNode{
				Val:  -4,
				Next: nil,
			},
		},
	}
	cycle1.Next.Next.Next = cycle1
	h1 := &ListNode{
		Val:  3,
		Next: cycle1,
	}

	h2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val:  2,
			Next: nil,
		},
	}
	h2.Next.Next = h2

	h3 := &ListNode{
		Val:  1,
		Next: nil,
	}

	// Expecting ALL true
	println(detectCycle(h1) == cycle1)
	println(detectCycle(h2) == h2)
	println(detectCycle(h3) == nil)
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	slow := head
	fast := head

	for fast != nil {
		slow = slow.Next
		if fast.Next == nil {
			return nil
		}

		fast = fast.Next.Next

		if slow == fast {
			cur := head
			for cur != slow {
				cur = cur.Next
				slow = slow.Next
			}

			return cur
		}
	}

	return nil
}
