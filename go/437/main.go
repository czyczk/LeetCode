package main

func main() {
	t1 := &TreeNode{
		Val: 10,
		Left: &TreeNode{
			Val: 5,
			Left: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 3,
				},
				Right: &TreeNode{
					Val: -2,
				},
			},
			Right: &TreeNode{
				Val: 2,
				Right: &TreeNode{
					Val: 1,
				},
			},
		},
		Right: &TreeNode{
			Val: -3,
			Right: &TreeNode{
				Val: 11,
			},
		},
	}

	t2 := &TreeNode{
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
				Left: &TreeNode{
					Val: 5,
				},
				Right: &TreeNode{
					Val: 1,
				},
			},
			Right: &TreeNode{
				Val: 4,
			},
		},
	}

	t3 := &TreeNode{
		Val: 1,
	}

	// Expecting 3
	println(pathSum(t1, 8))
	// Expecting 3
	println(pathSum(t2, 22))
	// Expecting 0
	println(pathSum(t3, 0))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var prefixSumMap map[int]int
var curSum int

func pathSum(root *TreeNode, targetSum int) int {
	prefixSumMap = make(map[int]int)
	prefixSumMap[0] = 1
	curSum = 0
	return preorder(root, targetSum)
}

func preorder(root *TreeNode, targetSum int) int {
	if root == nil {
		return 0
	}

	ret := 0
	curSum += root.Val

	existingNumPaths, ok := prefixSumMap[curSum-targetSum]
	if ok {
		ret += existingNumPaths
	}

	prefixSumMap[curSum]++

	ret += preorder(root.Left, targetSum)
	ret += preorder(root.Right, targetSum)

	prefixSumMap[curSum]--
	curSum -= root.Val

	return ret
}
