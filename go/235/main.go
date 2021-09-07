package main

func main() {
	t1 := &TreeNode{
		Val: 6,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 0,
			},
			Right: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 3,
				},
				Right: &TreeNode{
					Val: 5,
				},
			},
		},
		Right: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 7,
			},
			Right: &TreeNode{
				Val: 9,
			},
		},
	}

	p1, q1 := t1.Left, t1.Right

	t2 := t1
	p2, q2 := t2.Left, t2.Left.Right

	t3 := &TreeNode{
		Val: 2,
		Left: &TreeNode{
			Val: 1,
		},
	}
	p3, q3 := t3, t3.Left

	// Expecting 6
	println(lowestCommonAncestor(t1, p1, q1).Val)
	// Expecting 2
	println(lowestCommonAncestor(t2, p2, q2).Val)
	// Expecting 2
	println(lowestCommonAncestor(t3, p3, q3).Val)

}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var isPFound bool
var isQFound bool

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	isPFound = false
	isQFound = false

	if root == nil || root == p || root == q {
		if root == p {
			isPFound = true
		} else if root == q {
			isQFound = true
		}

		return root
	}

	var leftResult *TreeNode
	var rightResult *TreeNode

	if (!isPFound && p.Val < root.Val) || (!isQFound && q.Val < root.Val) {
		leftResult = lowestCommonAncestor(root.Left, p, q)
	}

	if (!isPFound && p.Val > root.Val) || (!isQFound && q.Val > root.Val) {
		rightResult = lowestCommonAncestor(root.Right, p, q)
	}

	if leftResult != nil && rightResult != nil {
		return root
	}

	if leftResult != nil {
		return leftResult
	}

	return rightResult

}
