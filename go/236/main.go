package main

func main() {
	p1 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 6,
		},
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 7,
			},
			Right: &TreeNode{
				Val: 4,
			},
		},
	}

	t1 := &TreeNode{
		Val:  3,
		Left: p1, Right: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val: 0,
			},
			Right: &TreeNode{
				Val: 8,
			},
		},
	}

	q1 := t1.Right

	t2 := t1
	p2 := t2.Left
	q2 := t2.Left.Right.Right

	// Expecting 3
	println(lowestCommonAncestor(t1, p1, q1).Val)
	// Expecting 5
	println(lowestCommonAncestor(t2, p2, q2).Val)
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	if root == nil || root == p || root == q {
		return root
	}

	leftResult := lowestCommonAncestor(root.Left, p, q)
	rightResult := lowestCommonAncestor(root.Right, p, q)
	if leftResult != nil && rightResult != nil {
		return root
	}

	if leftResult != nil {
		return leftResult
	}

	return rightResult
}
