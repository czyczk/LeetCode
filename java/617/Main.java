import java.util.ArrayDeque;
import java.util.Optional;
import java.util.Queue;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t11 = new TreeNode(1, new TreeNode(3, new TreeNode(5), null), new TreeNode(2));
		final var t12 = new TreeNode(2, new TreeNode(1, null, new TreeNode(4)), new TreeNode(3, null, new TreeNode(7)));

		final var t21 = new TreeNode(1);
		final var t22 = new TreeNode(1, new TreeNode(2), null);

		// Expecting [3, 4, 5, 5, 4, null, 7]
		printTree(s.mergeTrees(t11, t12));

		// Expecting [2, 2]
		printTree(s.mergeTrees(t21, t22));
	}

	private static void printTree(final TreeNode root) {
		if (root == null) {
			System.out.println("[]");
			return;
		}

		final Queue<Optional<TreeNode>> q = new ArrayDeque<>();
		q.offer(Optional.of(root));

		System.out.print("[");
		var isFirst = true;
		while (q.size() != 0) {
			final var layerSize = q.size();
			var isNextLayerAllNull = true;

			for (var i = 0; i < layerSize; i++) {
				final var node = q.poll();
				if (!isFirst) {
					System.out.print(", ");
				}

				if (node.isEmpty()) {
					System.out.print("null");
				} else {
					System.out.print(node.get().val);
					q.offer(Optional.ofNullable(node.get().left));
					q.offer(Optional.ofNullable(node.get().right));
					if (isNextLayerAllNull && (node.get().left != null || node.get().right != null)) {
						isNextLayerAllNull = false;
					}
				}

				isFirst = false;
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
	public TreeNode mergeTrees(final TreeNode root1, final TreeNode root2) {
		final var root = buildRoot(root1, root2);
		return root;
	}

	private TreeNode buildRoot(final TreeNode r1, final TreeNode r2) {
		if (r1 == null && r2 == null) {
			return null;
		}

		final var root = new TreeNode();
		if (r1 != null && r2 != null) {
			root.val = r1.val + r2.val;
			root.left = buildRoot(r1.left, r2.left);
			root.right = buildRoot(r1.right, r2.right);
		} else if (r1 != null) {
			root.val = r1.val;
			root.left = buildRoot(r1.left, null);
			root.right = buildRoot(r1.right, null);
		} else {
			root.val = r2.val;
			root.left = buildRoot(null, r2.left);
			root.right = buildRoot(null, r2.right);
		}

		return root;
	}
}
