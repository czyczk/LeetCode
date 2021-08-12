package main

import "fmt"

func main() {
	t1 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 11,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
		},
		Right: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 13,
			},
			Right: &TreeNode{
				Val: 4,
				Left: &TreeNode{
					Val: 5,
				},
				Right: &TreeNode{
					Val: 1,
				},
			},
		},
	}

	t2 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	t3 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
	}

	var t4 *TreeNode

	t5 := &TreeNode{
		Val: -2,
		Right: &TreeNode{
			Val: -3,
		},
	}

	// Expecting [[5, 4, 11, 2], [5, 8, 4, 5]]
	fmt.Printf("%v\n", pathSum(t1, 22))
	// Expecting []
	fmt.Printf("%v\n", pathSum(t2, 5))
	// Expecting []
	fmt.Printf("%v\n", pathSum(t3, 0))
	// Expecting []
	fmt.Printf("%v\n", pathSum(t4, 0))
	// Expecting [[-2, -3]]
	fmt.Printf("%v\n", pathSum(t5, -5))

}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var sum int
var trace []int
var ans [][]int

func pathSum(root *TreeNode, targetSum int) [][]int {
	sum = 0
	trace = []int{}
	ans = [][]int{}
	backtraceRec(root, targetSum)

	return ans
}

func backtraceRec(cur *TreeNode, targetSum int) {
	if cur == nil {
		return
	}

	sum += cur.Val
	trace = append(trace, cur.Val)

	if sum == targetSum && cur.Left == nil && cur.Right == nil {
		ans = append(ans, append([]int{}, trace...))
	}
	backtraceRec(cur.Left, targetSum)
	backtraceRec(cur.Right, targetSum)

	sum -= cur.Val
	trace = trace[:len(trace)-1]
}
