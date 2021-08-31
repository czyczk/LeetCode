public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t1 = new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7)));
		final var t2 = new TreeNode(2, null,
				new TreeNode(3, null, new TreeNode(4, null, new TreeNode(5, null, new TreeNode(6)))));
		final var t3 = new TreeNode(1);
		final TreeNode t4 = null;

		// Expecting 2
		System.out.println(s.minDepth(t1));
		// Expecting 5
		System.out.println(s.minDepth(t2));
		// Expecting 1
		System.out.println(s.minDepth(t3));
		// Expecting 0
		System.out.println(s.minDepth(t4));
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
	private int minDepth;

	public int minDepth(final TreeNode root) {
		if (root == null) {
			return 0;
		}

		minDepth = Integer.MAX_VALUE;
		traverse(root, 0);
		return minDepth;
	}

	private void traverse(final TreeNode cur, int curDepth) {
		if (cur == null) {
			return;
		}

		curDepth++;
		if (cur.left == null && cur.right == null) {
			minDepth = Math.min(minDepth, curDepth);
			return;
		}

		traverse(cur.left, curDepth);
		traverse(cur.right, curDepth);
	}
}
