package main

func main() {
	t1 := toTree([]nullableInt{newInt(1), newNil(), newInt(0), newInt(0), newInt(1)})
	t2 := toTree([]nullableInt{newInt(1), newInt(0), newInt(1), newInt(0), newInt(0), newInt(0), newInt(1)})
	t3 := toTree([]nullableInt{newInt(1), newInt(1), newInt(0), newInt(1), newInt(1), newInt(0), newInt(1), newInt(0)})

	// Expecting [1, nil, 0, nil, 1]
	pruneTree(t1)
	printTree(t1)

	// Expecting [1, nil, 1, nil, 1]
	pruneTree(t2)
	printTree(t2)

	// Expecting [1, 1, 0, 1, 1, nil, 1]
	pruneTree(t3)
	printTree(t3)
}

type nullableInt struct {
	val   int
	isNil bool
}

func newInt(val int) nullableInt {
	return nullableInt{
		val:   val,
		isNil: false,
	}
}

func newNil() nullableInt {
	return nullableInt{
		isNil: true,
	}
}

func toTree(arr []nullableInt) *TreeNode {
	n := len(arr)
	if n == 0 {
		return nil
	}

	if n == 1 && arr[0].isNil {
		return nil
	}

	root := &TreeNode{
		Val: arr[0].val,
	}

	var q []*TreeNode
	q = append(q, root)

	i := 1
	for i < n {
		node := q[0]
		q = q[1:]

		if !arr[i].isNil {
			node.Left = &TreeNode{
				Val: arr[i].val,
			}
			q = append(q, node.Left)
		}

		if i+1 < n && !arr[i+1].isNil {
			node.Right = &TreeNode{
				Val: arr[i+1].val,
			}
			q = append(q, node.Right)
		}

		i += 2
	}

	return root
}

func printTree(root *TreeNode) {
	if root == nil {
		return
	}

	isFirst := true

	var q []*TreeNode
	q = append(q, root)

	for len(q) > 0 {
		l := len(q)
		isAllLeaf := true

		for i := 0; i < l; i++ {
			if !isFirst {
				print(", ")
			}

			node := q[i]
			if node == nil {
				print("nil")
			} else {
				print(node.Val)
				if node.Left != nil || node.Right != nil {
					isAllLeaf = false
				}

				q = append(q, node.Left)
				q = append(q, node.Right)
			}

			isFirst = false
		}

		if isAllLeaf {
			break
		}

		q = q[l:]
	}

	println()
}

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pruneTree(root *TreeNode) *TreeNode {
	canTrim := canTrim(root)
	if canTrim {
		root = nil
	}

	return root
}

func canTrim(cur *TreeNode) bool {
	if cur == nil {
		return true
	}

	leftCanTrim := canTrim(cur.Left)
	if leftCanTrim {
		cur.Left = nil
	}

	rightCanTrim := canTrim(cur.Right)
	if rightCanTrim {
		cur.Right = nil
	}

	if leftCanTrim && rightCanTrim && cur.Val == 0 {
		return true
	}

	return false
}
