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
		Val: 3,
	}

	// Expecting [3, 2, 1]
	fmt.Printf("%v\n", postorderTraversal(r1))
	// Expecting []
	fmt.Printf("%v\n", postorderTraversal(r2))
	// Expecting [3]
	fmt.Printf("%v\n", postorderTraversal(r3))
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

func postorderTraversal(root *TreeNode) []int {
	ret = []int{}
	traverse(root)
	return ret
}

func traverse(node *TreeNode) {
	if node == nil {
		return
	}

	traverse(node.Left)
	traverse(node.Right)
	ret = append(ret, node.Val)
}
