package main

func main() {
	i1 := []int{9, 3, 15, 20, 7}
	p1 := []int{9, 15, 7, 20, 3}

	i2 := []int{-1}
	p2 := []int{-1}

	i3 := []int{}
	p3 := []int{}

	// Expecting [3, 9, 20, nil, nil, 15, 7]
	printTree(buildTree(i1, p1))

	// Expecting [-1]
	printTree(buildTree(i2, p2))

	// Expecting []
	printTree(buildTree(i3, p3))
}

func printTree(root *TreeNode) {
	if root == nil {
		println("[]")
		return
	}

	isFirst := true
	print("[")

	var q []*TreeNode
	q = append(q, root)

	for len(q) != 0 {
		layerSize := len(q)
		isNextLayerAllNil := true

		for i := 0; i < layerSize; i++ {
			if !isFirst {
				print(", ")
			}

			node := q[i]
			if node == nil {
				print("nil")
			} else {
				print(node.Val)
				q = append(q, node.Left)
				q = append(q, node.Right)
				if isNextLayerAllNil && (node.Left != nil || node.Right != nil) {
					isNextLayerAllNil = false
				}
			}

			isFirst = false
		}

		q = q[layerSize:]

		if isNextLayerAllNil {
			break
		}
	}

	println("]")
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(inorder []int, postorder []int) *TreeNode {
	root := getRoot(inorder, postorder)
	return root
}

func getRoot(inorder []int, postorder []int) *TreeNode {
	if len(inorder) == 0 {
		return nil
	}

	rootVal := postorder[len(postorder)-1]
	inorderSplitIdx := 0
	for inorderSplitIdx < len(inorder) {
		if inorder[inorderSplitIdx] == rootVal {
			break
		}
		inorderSplitIdx++
	}

	root := &TreeNode{
		Val: rootVal,
	}
	root.Left = getRoot(inorder[:inorderSplitIdx], postorder[:inorderSplitIdx])
	root.Right = getRoot(inorder[inorderSplitIdx+1:], postorder[inorderSplitIdx:len(postorder)-1])

	return root
}
