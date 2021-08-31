import java.util.ArrayDeque;
import java.util.Optional;
import java.util.Queue;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var p1 = new int[] { 3, 9, 20, 15, 7 };
		final var i1 = new int[] { 9, 3, 15, 20, 7 };

		// Expecting [3, 9, 20, null, null, 15, 7]
		printTree(s.buildTree(p1, i1));
	}

	private static void printTree(final TreeNode root) {
		if (root == null) {
			System.out.println("[]");
			return;
		}

		System.out.print("[");
		var isFirst = true;

		final Queue<Optional<TreeNode>> q = new ArrayDeque<>();
		q.offer(Optional.of(root));

		while (!q.isEmpty()) {
			final var layerLen = q.size();
			var isNextLayerAllEmpty = true;

			for (var i = 0; i < layerLen; i++) {
				if (!isFirst) {
					System.out.print(", ");
				}

				final var node = q.poll();
				if (node.isEmpty()) {
					System.out.print("null");
				} else {
					final var unwrappedNode = node.get();
					System.out.print(unwrappedNode.val);
					q.offer(Optional.ofNullable(unwrappedNode.left));
					q.offer(Optional.ofNullable(unwrappedNode.right));
					if (isNextLayerAllEmpty && (unwrappedNode.left != null || unwrappedNode.right != null)) {
						isNextLayerAllEmpty = false;
					}
				}

				isFirst = false;
			}

			if (isNextLayerAllEmpty) {
				break;
			}
		}

		System.out.println("]");
	}
}

/**
 * Definition for a binary tree node.
 */
class TreeNode {
	int val;
	TreeNode left;
	TreeNode right;

	TreeNode() {
	}

	TreeNode(final int val) {
		this.val = val;
	}

	TreeNode(final int val, final TreeNode left, final TreeNode right) {
		this.val = val;
		this.left = left;
		this.right = right;
	}
}

class Solution {
	public TreeNode buildTree(final int[] preorder, final int[] inorder) {
		final var root = getRoot(preorder, inorder, 0, preorder.length - 1, 0, inorder.length - 1);
		return root;
	}

	// preorderRight and inorderRight are both inclusive
	private TreeNode getRoot(final int[] preorder, final int[] inorder, final int preorderLeft, final int preorderRight, final int inorderLeft,
			final int inorderRight) {
		if (inorderLeft > inorderRight) {
			return null;
		}

		final var rootVal = preorder[preorderLeft];
		var inorderSplitIdx = inorderLeft;
		while (inorderSplitIdx <= inorderRight) {
			if (inorder[inorderSplitIdx] == rootVal) {
				break;
			}

			inorderSplitIdx++;
		}

		final var leftCount = inorderSplitIdx - inorderLeft;
		final var root = new TreeNode(rootVal,
				getRoot(preorder, inorder, preorderLeft + 1, preorderLeft + leftCount, inorderLeft,
						inorderSplitIdx - 1),
				getRoot(preorder, inorder, preorderLeft + leftCount + 1, preorderRight, inorderSplitIdx + 1,
						inorderRight));
		return root;
	}
}
