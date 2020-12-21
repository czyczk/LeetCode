import java.util.List;

public class Main {
    public static void main(String[] args) {
        var tree1 = new TreeNode(3);
        tree1.left = new TreeNode(9);
        tree1.right = new TreeNode(20);
        tree1.right.left = new TreeNode(15);
        tree1.right.right = new TreeNode(7);

        var s = new Solution();
        printList(s.zigzagLevelOrder(tree1));
    }

    private static void printList(List<List<Integer>> list) {
        System.out.println("[");

        for (var row : list) {
            System.out.print("\t[ ");
            for (var num : row) {
                System.out.print(num + " ");
            }
            System.out.println("]");
        }

        System.out.println("]");
    }
}