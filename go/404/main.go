package main

func main() {
	t1 := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 9,
		},
		Right: &TreeNode{
			Val: 20,
			Left: &TreeNode{
				Val: 15,
			},
			Right: &TreeNode{
				Val: 7,
			},
		},
	}

	t2 := &TreeNode{
		Val: 1,
	}

	// Expecting 24
	println(sumOfLeftLeaves(t1))
	// Expecting 0
	println(sumOfLeftLeaves(t2))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var sum int

func sumOfLeftLeaves(root *TreeNode) int {
	sum = 0
	traverse(root, false)
	return sum
}

func traverse(cur *TreeNode, isLeft bool) {
	if cur == nil {
		return
	}

	if isLeft && cur.Left == nil && cur.Right == nil {
		sum += cur.Val
	}

	traverse(cur.Left, true)
	traverse(cur.Right, false)
}
