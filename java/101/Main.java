import java.util.ArrayDeque;
import java.util.Optional;
import java.util.Queue;
import java.util.Stack;

public class Main {
	public static void main(String[] args) {
		var s = new Solution();

		var t1 = new TreeNode(1, new TreeNode(2, new TreeNode(3), new TreeNode(4)),
				new TreeNode(2, new TreeNode(4), new TreeNode(3)));
		var t2 = new TreeNode(1, new TreeNode(2, null, new TreeNode(3)), new TreeNode(2, null, new TreeNode(3)));

		// Expecting true
		System.out.println(s.isSymmetric(t1));
		// Expecting false
		System.out.println(s.isSymmetric(t2));
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

	TreeNode(int val) {
		this.val = val;
	}

	TreeNode(int val, TreeNode left, TreeNode right) {
		this.val = val;
		this.left = left;
		this.right = right;
	}
}

class Solution {
	public boolean isSymmetric(TreeNode root) {
		if (root == null) {
			return true;
		}

		Queue<Optional<TreeNode>> q = new ArrayDeque<Optional<TreeNode>>();
		q.offer(Optional.ofNullable(root.left));
		q.offer(Optional.ofNullable(root.right));

		while (!q.isEmpty()) {
			var len = q.size();

			var stack = new Stack<Optional<TreeNode>>();
			for (var i = 0; i < len; i++) {
				var node = q.poll();
				if (!node.isEmpty()) {
					q.offer(Optional.ofNullable(node.get().left));
					q.offer(Optional.ofNullable(node.get().right));
				}

				if (i < len / 2) {
					stack.push(node);
				} else {
					var nodeFromStack = stack.pop().orElse(null);
					var nodeFromQueue = node.orElse(null);
					if (nodeFromStack == null && nodeFromQueue != null
							|| nodeFromStack != null && nodeFromQueue == null) {
						return false;
					} else if (nodeFromStack == null && nodeFromQueue == null) {
						continue;
					} else if (nodeFromStack.val != nodeFromQueue.val) {
						return false;
					}
				}
			}
		}

		return true;
	}
}
