package main

func main() {
	t1, st1 := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 1,
			},
			Right: &TreeNode{
				Val: 2,
			},
		},
		Right: &TreeNode{
			Val: 5,
		},
	}, &TreeNode{
		Val: 4,
		Left: &TreeNode{
			Val: 1,
		},
		Right: &TreeNode{
			Val: 2,
		},
	}

	t2, st2 := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 1,
			},
			Right: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 0,
				},
			},
		},
		Right: &TreeNode{
			Val: 5,
		},
	}, st1

	// Expecting true
	println(isSubtree(t1, st1))
	// Expecting false
	println(isSubtree(t2, st2))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isNodeValEqual(node1 *TreeNode, node2 *TreeNode) bool {
	if node1 == nil && node2 == nil {
		return true
	}

	if node1 != nil && node2 != nil && node1.Val == node2.Val {
		return true
	}

	return false
}

func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	var treeNodes []*TreeNode
	var subtreeNodes []*TreeNode
	traversePreorder(root, &treeNodes)
	traversePreorder(subRoot, &subtreeNodes)
	next := buildNext(subtreeNodes)

	n := len(treeNodes)
	j := 0
	for i := 0; i < n; i++ {
		for j != 0 && !isNodeValEqual(treeNodes[i], subtreeNodes[j]) {
			j = next[j-1]
		}

		if isNodeValEqual(treeNodes[i], subtreeNodes[j]) {
			j++

			if j == len(subtreeNodes) {
				return true
			}
		}
	}

	return false
}

func traversePreorder(cur *TreeNode, result *[]*TreeNode) {
	if cur == nil {
		*result = append(*result, nil)
		return
	}

	*result = append(*result, cur)
	traversePreorder(cur.Left, result)
	traversePreorder(cur.Right, result)
}

func buildNext(nodes []*TreeNode) []int {
	n := len(nodes)
	next := make([]int, n)
	j := 0

	for i := 1; i < n; i++ {
		for j != 0 && !isNodeValEqual(nodes[i], nodes[j]) {
			j = next[j-1]
		}

		if isNodeValEqual(nodes[i], nodes[j]) {
			j++
		}

		next[i] = j
	}

	return next
}
