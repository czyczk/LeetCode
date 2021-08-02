import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        var s = new Solution();

        var r1 = new Node(1,
                Arrays.asList(new Node(3, Arrays.asList(new Node(5), new Node(6))), new Node(2), new Node(4)));

        var r2 = new Node(1,
                Arrays.asList(new Node(2),
                        new Node(3,
                                Arrays.asList(new Node(6),
                                        new Node(7, Arrays.asList(new Node(11, Arrays.asList(new Node(14))))))),
                        new Node(4, Arrays.asList(new Node(8, Arrays.asList(new Node(12))))),
                        new Node(5, Arrays.asList(new Node(9, Arrays.asList(new Node(13))), new Node(10)))));

        // Expecting [1, 3, 5, 6, 2, 4]
        System.out.println(s.preorder(r1));
        // Expecting [1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10]
        System.out.println(s.preorder(r2));
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

    public Node(int _val) {
        val = _val;
    }

    public Node(int _val, List<Node> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
    private List<Integer> ret;

    public List<Integer> preorder(Node root) {
        this.ret = new ArrayList<>();
        traverseRec(root);
        return this.ret;
    }

    private void traverseRec(Node node) {
        if (node == null) {
            return;
        }

        this.ret.add(node.val);
        if (node.children == null) {
            return;
        }

        for (var child : node.children) {
            traverseRec(child);
        }
    }
}