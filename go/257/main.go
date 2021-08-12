package main

import (
	"fmt"
	"strings"
)

func main() {
	t1 := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 5,
			},
		},
		Right: &TreeNode{
			Val: 3,
		},
	}

	// Expecting ["1->2->5", "1->3"]
	fmt.Printf("%v\n", binaryTreePaths(t1))
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

var ans []string
var trace []int

func binaryTreePaths(root *TreeNode) []string {
	ans = []string{}
	trace = []int{}

	if root == nil {
		return ans
	}

	backtraceRec(root)
	return ans
}

func backtraceRec(cur *TreeNode) {
	if cur == nil {
		return
	}

	trace = append(trace, cur.Val)
	if cur.Left == nil && cur.Right == nil {
		ans = append(ans, toAnsStr(trace))
	}
	backtraceRec(cur.Left)
	backtraceRec(cur.Right)
	trace = trace[:len(trace)-1]
}

func toAnsStr(trace []int) string {
	builder := strings.Builder{}
	isFirst := true

	for _, num := range trace {
		if !isFirst {
			builder.WriteString("->")
		}
		builder.WriteString(fmt.Sprintf("%v", num))
		isFirst = false
	}

	return builder.String()
}
