package main

import "fmt"

func main() {
	root1 := &TreeNode{
		Val: 4,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   3,
				Left:  nil,
				Right: nil,
			},
		},
		Right: &TreeNode{
			Val:   6,
			Left:  nil,
			Right: nil,
		},
	}

	root2 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val:   0,
			Left:  nil,
			Right: nil,
		},
		Right: &TreeNode{
			Val: 48,
			Left: &TreeNode{
				Val:   12,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Val:   49,
				Left:  nil,
				Right: nil,
			},
		},
	}

	root3 := &TreeNode{
		Val: 90,
		Left: &TreeNode{
			Val: 69,
			Left: &TreeNode{
				Val:  49,
				Left: nil,
				Right: &TreeNode{
					Val:   52,
					Left:  nil,
					Right: nil,
				},
			},
			Right: &TreeNode{
				Val:   89,
				Left:  nil,
				Right: nil,
			},
		},
		Right: nil,
	}

	// Expecting 1
	fmt.Println(minDiffInBST(root1))

	// Expecting 1
	fmt.Println(minDiffInBST(root2))

	// Expecting 1
	fmt.Println(minDiffInBST(root3))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var last int
var minDiff int

func minDiffInBST(root *TreeNode) int {
	last = -1
	minDiff = 100000
	inorder(root)
	return minDiff
}

func inorder(cur *TreeNode) {
	if cur == nil {
		return
	}

	inorder(cur.Left)
	// Do something with the current value
	if last != -1 {
		minDiff = min(abs(cur.Val-last), minDiff)
	}
	last = cur.Val

	inorder(cur.Right)
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func abs(a int) int {
	if a < 0 {
		return -a
	}

	return a
}
