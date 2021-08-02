import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.Queue;

public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var r1 = new TreeNode(3, new TreeNode(9), new TreeNode(20, new TreeNode(15), new TreeNode(7)));
        final var r2 = new TreeNode(3, new TreeNode(9, new TreeNode(15), new TreeNode(7)), new TreeNode(20));
        final var r3 = new TreeNode(2147483647, new TreeNode(2147483647), new TreeNode(2147483647));

        // Expecting [3, 14.5, 11]
        System.out.println(s.averageOfLevels(r1));
        // Expecting [3, 14.5, 11]
        System.out.println(s.averageOfLevels(r2));
        // Expecting [2147483647, 2147483647]
        System.out.println(s.averageOfLevels(r3));
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
    public List<Double> averageOfLevels(final TreeNode root) {
        final var ret = new ArrayList<Double>();
        if (root == null) {
            return ret;
        }

        final Queue<TreeNode> q = new ArrayDeque<TreeNode>();
        q.offer(root);
        while (!q.isEmpty()) {
            final var len = q.size();
            long sum = 0;
            for (var i = 0; i < len; i++) {
                final var node = q.poll();
                sum += node.val;
                if (node.left != null) {
                    q.offer(node.left);
                }
                if (node.right != null) {
                    q.offer(node.right);
                }
            }

            ret.add((double) sum / (double) len);
        }

        return ret;
    }
}