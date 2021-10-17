package main

func main() {
	t1 := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 1,
			Right: &TreeNode{
				Val: 2,
			},
		},
		Right: &TreeNode{
			Val: 4,
		},
	}

	t2 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 1,
				},
			},
			Right: &TreeNode{
				Val: 4,
			},
		},
		Right: &TreeNode{
			Val: 6,
		},
	}

	// Expecting 1
	println(kthSmallest(t1, 1))
	// Expecting 3
	println(kthSmallest(t2, 3))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var count int
var ans int

func kthSmallest(root *TreeNode, k int) int {
	count = 0
	ans = 0
	inorder(root, k)
	return ans
}

func inorder(root *TreeNode, k int) {
	if root == nil {
		return
	}

	inorder(root.Left, k)
	if count == k {
		return
	}

	ans = root.Val
	count++
	if count == k {
		return
	}

	inorder(root.Right, k)
}
