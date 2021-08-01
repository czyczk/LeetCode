package main

import "fmt"

func main() {
	r1 := &TreeNode{
		Val: 1,
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 3,
			},
		},
	}

	var r2 *TreeNode

	r3 := &TreeNode{
		Val: 1,
	}

	// Expecting [1, 3, 2]
	fmt.Printf("%v\n", inorderTraversal(r1))
	// Expecting []
	fmt.Printf("%v\n", inorderTraversal(r2))
	// Expecting [1]
	fmt.Printf("%v\n", inorderTraversal(r3))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var ret []int

func inorderTraversal(root *TreeNode) []int {
	ret = []int{}
	traverse(root)
	return ret
}

func traverse(node *TreeNode) {
	if node == nil {
		return
	}

	traverse(node.Left)
	ret = append(ret, node.Val)
	traverse(node.Right)
}
