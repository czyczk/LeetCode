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
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 4,
				},
				Right: &TreeNode{
					Val: 4,
				},
			},
			Right: &TreeNode{
				Val: 3,
			},
		},
		Right: &TreeNode{
			Val: 2,
		},
	}

	var t3 *TreeNode

	// Expecting true
	println(isBalanced(t1))
	// Expecting false
	println(isBalanced(t2))
	// Expecting true
	println(isBalanced(t3))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}

	depth := getDepth(root)
	return depth != -1
}

func getDepth(cur *TreeNode) int {
	if cur == nil {
		return 0
	}

	depthLeft := getDepth(cur.Left)
	if depthLeft == -1 {
		return -1
	}

	depthRight := getDepth(cur.Right)
	if depthRight == -1 {
		return -1
	}

	if abs(depthRight-depthLeft) > 1 {
		return -1
	}

	return 1 + max(depthLeft, depthRight)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

func max(x, y int) int {
	if x > y {
		return x
	}

	return y
}
