public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t1 = new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7)));
		final TreeNode t2 = null;
		final var t3 = new TreeNode(3);

		// Expecting 3
		System.out.println(s.maxDepth(t1));
		// Expecting 0
		System.out.println(s.maxDepth(t2));
		// Expecting 1
		System.out.println(s.maxDepth(t3));
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
	private int maxDepth;

	public int maxDepth(final TreeNode root) {
		this.maxDepth = 0;
		traverse(root, 0);

		return maxDepth;
	}

	private void traverse(final TreeNode cur, int curDepth) {
		if (cur == null) {
			return;
		}

		curDepth++;
		maxDepth = Math.max(maxDepth, curDepth);
		traverse(cur.left, curDepth);
		traverse(cur.right, curDepth);
	}
}
