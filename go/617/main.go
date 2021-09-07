package main

func main() {
	t11 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 2,
		},
	}

	t12 := &TreeNode{
		Val: 2,
		Left: &TreeNode{
			Val: 1,
			Right: &TreeNode{
				Val: 4,
			},
		},
		Right: &TreeNode{
			Val: 3,
			Right: &TreeNode{
				Val: 7,
			},
		},
	}

	t21 := &TreeNode{
		Val: 1,
	}

	t22 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
	}

	// Expecting [3, 4, 5, 5, 4, nil, 7]
	printTree(mergeTrees(t11, t12))
	// Expecting [2, 2]
	printTree(mergeTrees(t21, t22))
}

func printTree(root *TreeNode) {
	if root == nil {
		println("[]")
		return
	}

	print("[")
	isFirst := true
	q := []*TreeNode{root}

	for len(q) != 0 {
		layerLen := len(q)
		isNextLayerAllNil := true

		for i := 0; i < layerLen; i++ {
			node := q[i]

			if !isFirst {
				print(", ")
			}

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

		q = q[layerLen:]

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

func mergeTrees(root1 *TreeNode, root2 *TreeNode) *TreeNode {
	root := buildTree(root1, root2)
	return root
}

func buildTree(root1, root2 *TreeNode) *TreeNode {
	if root1 == nil && root2 == nil {
		return nil
	}

	if root1 != nil && root2 != nil {
		return &TreeNode{
			Val:   root1.Val + root2.Val,
			Left:  buildTree(root1.Left, root2.Left),
			Right: buildTree(root1.Right, root2.Right),
		}
	} else if root1 != nil {
		return &TreeNode{
			Val:   root1.Val,
			Left:  buildTree(root1.Left, nil),
			Right: buildTree(root1.Right, nil),
		}
	} else {
		return &TreeNode{
			Val:   root2.Val,
			Left:  buildTree(nil, root2.Left),
			Right: buildTree(nil, root2.Right),
		}
	}
}
