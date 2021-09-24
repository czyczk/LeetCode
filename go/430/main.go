package main

import "fmt"

func main() {
	h1 := buildH1()
	h2 := buildH2()
	var h3 *Node

	r1 := flatten(h1)
	// Expecting [1, 2, 3, 7, 8, 11, 12, 9, 10, 4, 5, 6]
	printFirstLayer(r1)
	ensureValidResult(r1)

	r2 := flatten(h2)
	// Expecting [1, 3, 2]
	printFirstLayer(r2)
	ensureValidResult(r2)

	r3 := flatten(h3)
	// Expecting []
	printFirstLayer(r3)
	ensureValidResult(r3)

}

func buildH1() *Node {
	layer1 := buildDoublyLinkedList([]int{1, 2, 3, 4, 5, 6})
	layer2 := buildDoublyLinkedList([]int{7, 8, 9, 10})
	layer3 := buildDoublyLinkedList([]int{11, 12})
	layer1.Next.Next.Child = layer2
	layer2.Next.Child = layer3
	return layer1
}

func buildH2() *Node {
	layer1 := buildDoublyLinkedList([]int{1, 2})
	layer2 := buildDoublyLinkedList([]int{3})
	layer1.Child = layer2
	return layer1
}

func buildDoublyLinkedList(nums []int) *Node {
	if len(nums) == 0 {
		return nil
	}

	head := &Node{
		Val: nums[0],
	}
	cur := head

	for i := 1; i < len(nums); i++ {
		newNode := &Node{
			Val:  nums[i],
			Prev: cur,
		}
		cur.Next = newNode
		cur = newNode
	}

	return head
}

func printFirstLayer(head *Node) {
	print("[")
	isFirst := true
	cur := head
	for cur != nil {
		if !isFirst {
			print(", ")
		} else {
			isFirst = false
		}

		print(cur.Val)
		cur = cur.Next
	}

	println("]")
}

func ensureValidDoublyLinkedList(head *Node) {
	if head == nil {
		return
	}

	q := []*Node{head}
	for len(q) != 0 {
		node := q[0]
		q = q[1:]

		if node.Child != nil {
			q = append(q, node.Child)
		}

		cur := node
		for cur != nil {
			next := cur.Next
			if next != nil && next.Prev != cur {
				panic(fmt.Sprintf("%v -> %v ✓. %v <- %v ✕.\n", cur.Val, next.Val, cur.Val, next.Val))
			}

			cur = next
		}
	}

	return
}

func ensureValidResult(head *Node) {
	if head == nil {
		return
	}

	ensureValidDoublyLinkedList(head)

	cur := head
	for cur != nil {
		if cur.Child != nil {
			panic(fmt.Sprintf("%v has a child node ✕.\n", cur.Val))
		}

		cur = cur.Next
	}
}

/**
 * Definition for a Node.
 */
type Node struct {
	Val   int
	Prev  *Node
	Next  *Node
	Child *Node
}

func flatten(root *Node) *Node {
	head := root
	flattenChild(head)

	return head
}

func flattenChild(head *Node) *Node {
	var prev *Node
	cur := head

	for cur != nil {
		if cur.Child == nil {
			prev = cur
			cur = cur.Next
			continue
		}

		next := cur.Next
		childFirst := cur.Child
		cur.Next = childFirst
		childFirst.Prev = cur
		cur.Child = nil

		prev = flattenChild(childFirst)
		prev.Next = next
		cur = next
		if cur != nil {
			cur.Prev = prev
		}
	}

	return prev
}
