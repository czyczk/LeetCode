import java.util.ArrayDeque;
import java.util.Optional;
import java.util.Queue;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t1 = new TreeNode(4, new TreeNode(2, new TreeNode(1), new TreeNode(3)), new TreeNode(7));
		final var t2 = new TreeNode(4, new TreeNode(2, new TreeNode(1), new TreeNode(3)), new TreeNode(7));
		final TreeNode t3 = null;

		// Expecting [2, 1, 3]
		printTree(s.searchBST(t1, 2));

		// Expecting []
		printTree(s.searchBST(t2, 5));

		// Expecting []
		printTree(s.searchBST(t3, 5));
	}

	private static void printTree(final TreeNode root) {
		if (root == null) {
			System.out.println("[]");
			return;
		}

		final Queue<Optional<TreeNode>> q = new ArrayDeque<>();
		q.offer(Optional.of(root));
		var isFirst = true;
		System.out.print("[");
		while (!q.isEmpty()) {
			final var layerSize = q.size();
			var isNextLayerAllNull = true;
			for (var i = 0; i < layerSize; i++) {
				if (!isFirst) {
					System.out.print(", ");
				}
				final var node = q.poll();
				if (node.isEmpty()) {
					System.out.print("null");
				} else {
					final var nodeUnwrapped = node.get();
					System.out.print(nodeUnwrapped.val);
					q.offer(Optional.ofNullable(nodeUnwrapped.left));
					q.offer(Optional.ofNullable(nodeUnwrapped.right));
					if (isNextLayerAllNull && (nodeUnwrapped.left != null || nodeUnwrapped.right != null)) {
						isNextLayerAllNull = false;
					}
					isFirst = false;
				}
			}

			if (isNextLayerAllNull) {
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
	public TreeNode searchBST(TreeNode root, final int val) {
		while (root != null) {
			if (root.val == val) {
				return root;
			} else if (root.val < val) {
				root = root.right;
			} else {
				root = root.left;
			}
		}

		return null;
	}
}
