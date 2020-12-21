import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Solution {
    public List<List<Integer>> zigzagLevelOrder(TreeNode root) {
        var result = new ArrayList<List<Integer>>();

        if (root == null) {
            return result;
        }

        var layer = 0;

        var queue = new LinkedList<TreeNode>();
        queue.add(root);

        while (!queue.isEmpty()) {
            var fromLeft = layer % 2 == 0;
            var layerNodes = new ArrayList<TreeNode>();
            while (!queue.isEmpty()) {
                layerNodes.add(queue.removeFirst());
            }

            var numLayerNodes = layerNodes.size();

            for (var node : layerNodes) {
                if (node.left != null) {
                    queue.add(node.left);
                }

                if (node.right != null) {
                    queue.add(node.right);
                }
            }

            var layerInts = new ArrayList<Integer>();
            for (var i = 0; i < numLayerNodes; i++) {
                if (fromLeft) {
                    layerInts.add(layerNodes.get(i).val);
                } else {
                    layerInts.add(layerNodes.get(numLayerNodes - i - 1).val);
                }
            }
            result.add(layerInts);

            layer++;
        }

        return result;
    }
}
