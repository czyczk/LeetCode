public class Main {
    public static void main(String[] args) {
        SolutionNormal s = new SolutionNormal();
        int count1 = s.countNodes(createRoot1());
        System.out.println(count1);
        int count2 = s.countNodes(createRoot2());
        System.out.println(count2);
    }

    public static TreeNode createRoot1() {
        TreeNode root1 = new TreeNode(1);
        TreeNode tempL = new TreeNode(4);
        TreeNode tempR = new TreeNode(5);
        TreeNode temp = new TreeNode(2);
        temp.left = tempL;
        temp.right = tempR;
        tempL = new TreeNode(6);
        tempR = new TreeNode(3);
        tempR.left = tempL;
        tempL = temp;
        root1.left = tempL;
        root1.right = tempR;

        return root1;
    }

    public static TreeNode createRoot2() {
        TreeNode root2 = new TreeNode(1);
        root2.left = new TreeNode(2);
        root2.right = new TreeNode(3);
        root2.left.left = new TreeNode(4);
        root2.left.right = new TreeNode(5);

        return root2;
    }
}