package main

import "fmt"

func main() {
	r1 := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 9,
		},
		Right: &TreeNode{
			Val: 20,
			Left: &TreeNode{
				Val: 15,
			},
			Right: &TreeNode{
				Val: 7,
			},
		},
	}

	r2 := &TreeNode{
		Val: 1,
	}

	var r3 *TreeNode

	// Expecting [[15, 7], [9, 20], [3]]
	fmt.Printf("%v\n", levelOrderBottom(r1))
	// Expecting [[1]]
	fmt.Printf("%v\n", levelOrderBottom(r2))
	// Expecting []
	fmt.Printf("%v\n", levelOrderBottom(r3))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrderBottom(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	var ret [][]int
	q := []*TreeNode{root}

	for len(q) > 0 {
		levelNum := len(q)
		var levelVals []int

		for i := 0; i < levelNum; i++ {
			node := q[i]
			levelVals = append(levelVals, node.Val)
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}

		ret = append(ret, levelVals)
		q = q[levelNum:]
	}

	reverse(ret)
	return ret
}

func reverse(arr [][]int) {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
}
