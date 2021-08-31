package main

func main() {
	t1 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 6,
			},
		},
	}

	var t2 *TreeNode

	// Expecting 6
	println(countNodes(t1))
	// Expecting 0
	println(countNodes(t2))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func countNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}

	depthLeft := getDepth(root.Left)
	depthRight := getDepth(root.Right)
	if depthLeft == depthRight {
		// Left subtree is full
		return (1 << depthLeft) + countNodes(root.Right)
	} else {
		// Right subtree is full
		return (1 << depthRight) + countNodes(root.Left)
	}
}

func getDepth(cur *TreeNode) int {
	if cur == nil {
		return 0
	}

	depth := 0
	for cur != nil {
		depth++
		cur = cur.Left
	}

	return depth
}
