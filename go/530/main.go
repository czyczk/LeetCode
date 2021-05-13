package main

import "fmt"

func main() {
	root := &TreeNode{
		Val:  1,
		Left: nil,
		Right: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val:   2,
				Left:  nil,
				Right: nil,
			},
			Right: nil,
		},
	}

	// Expecting 1
	fmt.Println(getMinimumDifference(root))
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

func getMinimumDifference(root *TreeNode) int {
	last = -1
	minDiff = 1000000

	inorder(root)
	return minDiff
}

func inorder(cur *TreeNode) {
	if cur == nil {
		return
	}

	inorder(cur.Left)

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

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}
