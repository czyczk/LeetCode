package main

import "fmt"

func main() {
	preorder := []int{3, 9, 20, 15, 7}
	inorder := []int{9, 3, 15, 20, 7}

	preorder2 := []int{1, 2}
	inorder2 := []int{1, 2}

	root := buildTree(preorder, inorder)
	printTreePreorder(root)
	fmt.Println()
	root2 := buildTree(preorder2, inorder2)
	printTreePreorder(root2)
	fmt.Println()
}

func printTreePreorder(root *TreeNode) {
	if root == nil {
		return
	}

	fmt.Printf("%v ", root.Val)
	printTreePreorder(root.Left)
	printTreePreorder(root.Right)
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
	if n == 0 {
		return nil
	}

	return buildSubtree(preorder, inorder, 0, n-1, 0, n-1)
}

func buildSubtree(pre []int, in []int, preStart int, preEnd int, inStart int, inEnd int) *TreeNode {
	// Special case: start > end
	if preStart > preEnd || inStart > inEnd {
		return nil
	}

	rootVal := pre[preStart]

	root := &TreeNode{
		Val:   rootVal,
		Left:  nil,
		Right: nil,
	}

	// Special case: only one element is left in range
	if preStart == preEnd {
		return root
	}

	// Find the root to split pre and in
	idxInRoot := -1
	for i := inStart; i <= inEnd; i++ {
		if in[i] == rootVal {
			idxInRoot = i
			break
		}
	}

	// If the root is not found in inorder, create a panic
	if idxInRoot == -1 {
		panic("Invalid input")
	}

	lenLeft := idxInRoot - inStart

	// Recursively build left and right subtrees
	root.Left = buildSubtree(pre, in, preStart+1, preStart+lenLeft, inStart, idxInRoot-1)
	root.Right = buildSubtree(pre, in, preStart+1+lenLeft, preEnd, idxInRoot+1, inEnd)

	return root
}
