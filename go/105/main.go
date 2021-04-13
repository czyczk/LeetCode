package main

import "fmt"

func main() {
	pre := []int{3, 9, 20, 15, 7}
	in := []int{9, 3, 15, 20, 7}

	root := buildTree(pre, in)
	printTree(root)
}

func printTree(root *TreeNode) {
	if root == nil {
		fmt.Print("nil ")
		return
	}

	fmt.Printf("%v ", root.Val)
	printTree(root.Left)
	printTree(root.Right)
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	n := len(preorder)
	return buildSubtree(preorder, inorder, 0, n-1, 0, n-1)
}

func buildSubtree(pre []int, in []int, preStart, preEnd, inStart, inEnd int) *TreeNode {
	// Special case: start > end
	if preStart > preEnd || inStart > inEnd {
		return nil
	}

	// Construct the root node
	rootVal := pre[preStart]
	root := &TreeNode{
		Val:   rootVal,
		Left:  nil,
		Right: nil,
	}

	// Special case: only one element is in range
	if preStart == preEnd {
		return root
	}

	// Find the idx of the root in in and split pre and in by the idx
	idxInRoot := -1
	for i := inStart; i <= inEnd; i++ {
		if in[i] == rootVal {
			idxInRoot = i
			break
		}
	}

	// Panic if the root value is not found in in
	if idxInRoot == -1 {
		panic("Invalid input")
	}

	lenLeft := idxInRoot - inStart

	root.Left = buildSubtree(pre, in, preStart+1, preStart+lenLeft, inStart, idxInRoot-1)
	root.Right = buildSubtree(pre, in, preStart+lenLeft+1, preEnd, idxInRoot+1, inEnd)

	return root
}
