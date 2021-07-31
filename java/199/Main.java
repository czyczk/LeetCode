import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.Queue;

public class Main {
	public static void main(String[] args) {
		var s = new Solution();

		List<Integer> l1 = Arrays.asList(1, 2, 3, null, 5, null, 4);
		var t1 = toTree(l1);

		List<Integer> l2 = Arrays.asList(1, null, 3);
		var t2 = toTree(l2);

		TreeNode t3 = null;

		// Expecting [1, 3, 4]
		System.out.println(s.rightSideView(t1));
		// Expecting [1, 3]
		System.out.println(s.rightSideView(t2));
		// Expecting []
		System.out.println(s.rightSideView(t3));
	}

	private static TreeNode toTree(List<Integer> list) {
		if (list == null || list.isEmpty()) {
			return null;
		}

		var root = new TreeNode(list.get(0));
		Queue<Optional<TreeNode>> q = new ArrayDeque<Optional<TreeNode>>();
		q.offer(Optional.of(root));

		for (int i = 1; i < list.size(); i += 2) {
			var node = q.poll().orElse(null);
			if (node == null) {
				q.offer(Optional.empty());
				q.offer(Optional.empty());
				continue;
			}

			var valLeft = list.get(i);
			if (valLeft != null) {
				node.left = new TreeNode(valLeft);
			}
			q.offer(Optional.ofNullable(node.left));

			Integer valRight = null;
			if (i + 1 < list.size()) {
				valRight = list.get(i + 1);
			}
			if (valRight != null) {
				node.right = new TreeNode(valRight);
			}
			q.offer(Optional.ofNullable(node.right));
		}

		return root;
	}
}
/**
 * Definition for a binary tree node.
 */
class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

class Solution {
    public List<Integer> rightSideView(TreeNode root) {
		if (root == null) {
			return new ArrayList<>();
		}

		Queue<TreeNode> q = new ArrayDeque<TreeNode>();
		q.offer(root);
		var ret = new ArrayList<Integer>();

		while (!q.isEmpty()) {
			var len = q.size();
			for (int i = 0; i < len; i++) {
				var node = q.poll();
				if (node.left != null) {
					q.offer(node.left);
				}

				if (node.right != null) {
					q.offer(node.right);
				}

				if (i == len - 1) {
					ret.add(node.val);
				}
			}
		}

		return ret;
    }
}
