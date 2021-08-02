import java.util.ArrayDeque;
import java.util.Queue;

public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var r1 = new TreeNode(4, new TreeNode(2, new TreeNode(1), new TreeNode(3)),
                new TreeNode(7, new TreeNode(6), new TreeNode(9)));

        final var r2 = new TreeNode(2, new TreeNode(1), new TreeNode(3));

        // Expecting [[4], [7, 2], [9, 6, 3, 1]]
        s.invertTree(r1);
        printByLayer(r1);

        // Expecting [[2], [3, 1]]
        s.invertTree(r2);
        printByLayer(r2);
    }

    private static void printByLayer(final TreeNode root) {
        if (root == null) {
            System.out.println("[]");
            return;
        }

        final Queue<TreeNode> q = new ArrayDeque<TreeNode>();
        q.offer(root);
        System.out.println("[");

        while (!q.isEmpty()) {
            final var len = q.size();
            System.out.printf("  [");
            for (var i = 0; i < len; i++) {
                final var node = q.poll();
                if (node.left != null) {
                    q.offer(node.left);
                }
                if (node.right != null) {
                    q.offer(node.right);
                }

                if (i > 0) {
                    System.out.print(", ");
                }
                System.out.print(node.val);
            }
            System.out.println("]");
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
    public TreeNode invertTree(final TreeNode root) {
        if (root == null) {
            return null;
        }

        final Queue<TreeNode> q = new ArrayDeque<TreeNode>();
        q.offer(root);

        while (!q.isEmpty()) {
            final var node = q.poll();
            if (node.left != null) {
                q.offer(node.left);
            }
            if (node.right != null) {
                q.offer(node.right);
            }

            final var temp = node.left;
            node.left = node.right;
            node.right = temp;
        }

        return root;
    }

}