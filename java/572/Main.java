import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var r1 = new TreeNode(3, new TreeNode(4, new TreeNode(1), new TreeNode(2)), new TreeNode(5));
		final var sr1 = new TreeNode(4, new TreeNode(1), new TreeNode(2));

		final var r2 = new TreeNode(3, new TreeNode(4, new TreeNode(1), new TreeNode(2, new TreeNode(0), null)),
				new TreeNode(5));
		final var sr2 = sr1;

		// Expecting true
		System.out.println(s.isSubtree(r1, sr1));

		// Expecting false
		System.out.println(s.isSubtree(r2, sr2));
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

	public String toString() {
		return ((Integer) val).toString();
	}
}

class Solution {
	public boolean isSubtree(final TreeNode root, final TreeNode subRoot) {
		final List<Optional<TreeNode>> treeNodes = new ArrayList<>();
		final List<Optional<TreeNode>> subtreeNodes = new ArrayList<>();
		traversePreorder(root, treeNodes);
		traversePreorder(subRoot, subtreeNodes);
		final var next = buildNext(subtreeNodes);

		final var n = treeNodes.size();
		var j = 0;
		for (var i = 0; i < n; i++) {
			while (j != 0 && !isNodeValEqual(treeNodes.get(i), subtreeNodes.get(j))) {
				j = next[j - 1];
			}

			if (isNodeValEqual(treeNodes.get(i), subtreeNodes.get(j))) {
				j++;

				if (j == subtreeNodes.size()) {
					return true;
				}
			}
		}

		return false;
	}

	private void traversePreorder(final TreeNode cur, final List<Optional<TreeNode>> nodes) {
		if (cur == null) {
			nodes.add(Optional.empty());
			return;
		}

		nodes.add(Optional.of(cur));
		traversePreorder(cur.left, nodes);
		traversePreorder(cur.right, nodes);
	}

	private int[] buildNext(final List<Optional<TreeNode>> subtreeNodes) {
		final var len = subtreeNodes.size();
		final var next = new int[len];

		var j = 0;
		for (var i = 1; i < len; i++) {
			while (j != 0 && !isNodeValEqual(subtreeNodes.get(i), subtreeNodes.get(j))) {
				j = next[j - 1];
			}

			if (isNodeValEqual(subtreeNodes.get(i), subtreeNodes.get(j))) {
				j++;
			}

			next[i] = j;
		}

		return next;
	}

	private boolean isNodeValEqual(final Optional<TreeNode> node1, final Optional<TreeNode> node2) {
		final var n1 = node1.orElse(null);
		final var n2 = node2.orElse(null);

		if (n1 == null && n2 == null) {
			return true;
		}

		if (n1 != null && n2 != null && n1.val == n2.val) {
			return true;
		}

		return false;
	}
}
