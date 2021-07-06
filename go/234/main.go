package main

func main() {
	h1 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val:  2,
			Next: nil,
		},
	}

	h2 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 2,
				Next: &ListNode{
					Val:  1,
					Next: nil,
				},
			},
		},
	}

	// Expecting false
	println(isPalindrome(h1))
	// Expecting true
	println(isPalindrome(h2))
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	n := 0
	cur := head
	headLatterHalf := head
	for cur != nil {
		n++
		if n%2 != 0 {
			headLatterHalf = headLatterHalf.Next
		}
		cur = cur.Next
	}

	if n == 1 {
		return true
	}

	var prev *ListNode
	cur = head
	numReversed := 0
	for numReversed < n/2 {
		next := cur.Next
		cur.Next = prev
		prev = cur
		cur = next
		numReversed++
	}

	curFirstHalf := prev
	curLatterHalf := headLatterHalf
	for curFirstHalf != nil {
		if curFirstHalf.Val != curLatterHalf.Val {
			return false
		}
		curFirstHalf = curFirstHalf.Next
		curLatterHalf = curLatterHalf.Next
	}

	return true
}
