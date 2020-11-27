public class Solution {
    public int countNodes(TreeNode root) {
        TreeNode cursor = root;
        int count = 0;

        while (cursor != null) {
            // Get the height of the left subtree.
            int heightLeftSubtree = getHeightOfSubtree(cursor.left);
            int heightRightSubtree = getHeightOfSubtree(cursor.right);

            // Height left == height right -> left is full.
            if (heightLeftSubtree == heightRightSubtree) {
                count += (1 << heightLeftSubtree);
                cursor = cursor.right;
            }
            // Height left > height right -> (height - 1) is full -> right is known.
            else {
                count += (1 << heightRightSubtree);
                cursor = cursor.left;
            }
        }

        return count;

    }

    private int getHeightOfSubtree(TreeNode subtreeRoot) {
        if (subtreeRoot == null) {
            return 0;
        }

        int height = 0;
        TreeNode cursor = subtreeRoot;
        while (cursor != null) {
            height++;
            cursor = cursor.left;
        }

        return height;
    }
}
