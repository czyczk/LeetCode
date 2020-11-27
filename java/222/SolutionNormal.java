import java.util.LinkedList;

public class SolutionNormal {
    public int countNodes(TreeNode root) {
        if (root == null) {
            return 0;
        }

        int count = 0;
        TreeNode cursor = root;
        LinkedList<TreeNode> stack = new LinkedList<>();
        stack.addLast(root);

        while (!stack.isEmpty()) {
            cursor = stack.removeFirst();
            count++;
            if (cursor.left != null) {
                stack.add(cursor.left);
            }
            if (cursor.right != null) {
                stack.add(cursor.right);
            }
        }

        return count;
    }
}