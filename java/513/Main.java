import java.util.ArrayDeque;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.Queue;

public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var l1 = Arrays.asList(2, 1, 3);
        final var t1 = toTree(l1);

        final var l2 = Arrays.asList(1, 2, 3, 4, null, 5, 6, null, null, 7);
        final var t2 = toTree(l2);

        // Expecting 1
        System.out.println(s.findBottomLeftValue(t1));
        // Expecting 7
        System.out.println(s.findBottomLeftValue(t2));
    }

    private static TreeNode toTree(final List<Integer> list) {
        if (list == null || list.size() == 0) {
            return null;
        }

        final var root = new TreeNode(list.get(0));
        final Queue<TreeNode> q = new ArrayDeque<TreeNode>();
        q.offer(root);

        for (int i = 1; i < list.size(); i += 2) {
            final var node = q.poll();

            final var valLeft = list.get(i);
            if (valLeft != null) {
                node.left = new TreeNode(valLeft);
                q.offer(node.left);
            }

            Integer valRight = null;
            if (i + 1 < list.size()) {
                valRight = list.get(i + 1);
            }
            if (valRight != null) {
                node.right = new TreeNode(valRight);
                q.offer(node.right);
            }
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
    public int findBottomLeftValue(final TreeNode root) {
        var ret = 0;
        final Queue<TreeNode> q = new ArrayDeque<TreeNode>();
        q.offer(root);

        while (!q.isEmpty()) {
            final var len = q.size();
            for (int i = 0; i < len; i++) {
                final var node = q.poll();

                if (i == 0) {
                    ret = node.val;
                }

                if (node.left != null) {
                    q.offer(node.left);
                }

                if (node.right != null) {
                    q.offer(node.right);
                }
            }
        }

        return ret;
    }
}