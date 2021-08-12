package main

func main() {
	t1 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 11,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
		},
		Right: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 13,
			},
			Right: &TreeNode{
				Val: 4,
				Right: &TreeNode{
					Val: 1,
				},
			},
		},
	}

	t2 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	t3 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
	}

	var t4 *TreeNode

	t5 := &TreeNode{
		Val: -2,
		Right: &TreeNode{
			Val: -3,
		},
	}

	// Expecting true
	println(hasPathSum(t1, 22))
	// Expecting false
	println(hasPathSum(t2, 5))
	// Expecting false
	println(hasPathSum(t3, 0))
	// Expecting false
	println(hasPathSum(t4, 0))
	// Expecting true
	println(hasPathSum(t5, -5))
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

func hasPathSum(root *TreeNode, targetSum int) bool {
	sum = 0
	isExisting := backtraceRec(root, targetSum)
	return isExisting
}

func backtraceRec(cur *TreeNode, targetSum int) bool {
	if cur == nil {
		return false
	}

	sum += cur.Val
	if sum == targetSum && cur.Left == nil && cur.Right == nil {
		return true
	}

	isExisting := backtraceRec(cur.Left, targetSum)
	if isExisting {
		return true
	}

	isExisting = backtraceRec(cur.Right, targetSum)
	if isExisting {
		return true
	}

	sum -= cur.Val
	return false
}
