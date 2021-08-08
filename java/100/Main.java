import java.util.ArrayDeque;
import java.util.Optional;
import java.util.Queue;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t1p = new TreeNode(1, new TreeNode(2), new TreeNode(3));
		final var t1q = new TreeNode(1, new TreeNode(2), new TreeNode(3));

		final var t2p = new TreeNode(1, new TreeNode(2), null);
		final var t2q = new TreeNode(1, null, new TreeNode(2));

		final var t3p = new TreeNode(1, new TreeNode(2), new TreeNode(1));
		final var t3q = new TreeNode(1, new TreeNode(1), new TreeNode(2));

		// Expecting true
		System.out.println(s.isSameTree(t1p, t1q));

		// Expecting false
		System.out.println(s.isSameTree(t2p, t2q));

		// Expecting false
		System.out.println(s.isSameTree(t3p, t3q));
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
	public boolean isSameTree(final TreeNode p, final TreeNode q) {
		final Queue<Optional<TreeNode>> que1 = new ArrayDeque<>();
		final Queue<Optional<TreeNode>> que2 = new ArrayDeque<>();
		que1.offer(Optional.ofNullable(p));
		que2.offer(Optional.ofNullable(q));

		while (!que1.isEmpty()) {
			final var len = que1.size();
			for (var i = 0; i < len; i++) {
				final var node1 = que1.poll().orElse(null);
				final var node2 = que2.poll().orElse(null);
				if (node1 == null && node2 != null || node1 != null && node2 == null) {
					return false;
				} else if (node1 != null && node2 != null && node1.val != node2.val) {
					return false;
				}

				if (node1 != null) {
					que1.offer(Optional.ofNullable(node1.left));
					que1.offer(Optional.ofNullable(node1.right));
					que2.offer(Optional.ofNullable(node2.left));
					que2.offer(Optional.ofNullable(node2.right));
				}
			}
		}
		return true;
	}
}
