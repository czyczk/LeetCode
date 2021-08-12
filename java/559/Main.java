import java.util.Arrays;
import java.util.List;

public class Main {
	public static void main(final String[] args) {
		final var s = new Solution();

		final var t1 = new Node(1,
				Arrays.asList(new Node(3, Arrays.asList(new Node(5), new Node(6))), new Node(2), new Node(4)));
		final var t2 = new Node(1,
				Arrays.asList(new Node(2),
						new Node(3,
								Arrays.asList(new Node(6),
										new Node(7, Arrays.asList(new Node(11, Arrays.asList(new Node(14))))))),
						new Node(4, Arrays.asList(new Node(8, Arrays.asList(new Node(12))))),
						new Node(5, Arrays.asList(new Node(9, Arrays.asList(new Node(13))), new Node(10)))));
		final var t3 = new Node(1);
		final Node t4 = null;

		// Expecting 3
		System.out.println(s.maxDepth(t1));
		// Expecting 5
		System.out.println(s.maxDepth(t2));
		// Expecting 1
		System.out.println(s.maxDepth(t3));
		// Expecting 0
		System.out.println(s.maxDepth(t4));
	}
}

/*
 * // Definition for a Node.
 */
class Node {
	public int val;
	public List<Node> children;

	public Node() {
	}

	public Node(final int _val) {
		val = _val;
	}

	public Node(final int _val, final List<Node> _children) {
		val = _val;
		children = _children;
	}
};

class Solution {
	private int maxDepth;

	public int maxDepth(final Node root) {
		maxDepth = 0;
		traverse(root, 0);
		return maxDepth;
	}

	private void traverse(final Node cur, int curDepth) {
		if (cur == null) {
			return;
		}

		curDepth++;
		maxDepth = Math.max(maxDepth, curDepth);
		if (cur.children != null) {
			for (final var child : cur.children) {
				traverse(child, curDepth);
			}
		}
	}
}
