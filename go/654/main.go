package main

func main() {
	n1 := []int{3, 2, 1, 6, 0, 5}
	n2 := []int{3, 2, 1}

	// Expecting [6, 3, 5, nil, 2, 0, nil, nil, 1]
	printTree(constructMaximumBinaryTree(n1))
	// Expecting [3, nil, 2, nil, 1]
	printTree(constructMaximumBinaryTree(n2))
}

func printTree(root *TreeNode) {
	if root == nil {
		println("[]")
		return
	}

	isFirst := true
	var q []*TreeNode
	q = append(q, root)
	print("[")

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
				continue
			}

			print(node.Val)

			q = append(q, node.Left)
			q = append(q, node.Right)
			if node.Left != nil || node.Right != nil {
				isNextLayerAllNil = false
			}
			isFirst = false
		}

		if isNextLayerAllNil {
			break
		}

		q = q[layerSize:]
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

func constructMaximumBinaryTree(nums []int) *TreeNode {
	return getRoot(nums)
}

func getRoot(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}

	maxIdx := getMaxIdx(nums)
	root := &TreeNode{
		Val:   nums[maxIdx],
		Left:  getRoot(nums[:maxIdx]),
		Right: getRoot(nums[maxIdx+1:]),
	}
	return root
}

func getMaxIdx(nums []int) int {
	idx := 0
	max := nums[0]

	for i, num := range nums {
		if num > max {
			max = num
			idx = i
		}
	}

	return idx
}
