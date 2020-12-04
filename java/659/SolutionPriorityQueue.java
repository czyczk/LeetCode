import java.util.HashMap;
import java.util.PriorityQueue;

public class SolutionPriorityQueue {
    public boolean isPossible(int[] nums) {
        var map = new HashMap<Integer, PriorityQueue<Integer>>();

        for (var num : nums) {
            var subSeqLens = map.get(num - 1);

            var maxLen = 0;
            if (subSeqLens == null || subSeqLens.isEmpty()) {
                maxLen++;
            } else {
                maxLen = subSeqLens.remove() + 1;
            }

            if (map.containsKey(num)) {
                map.get(num).offer(maxLen);
            } else {
                var prioQueue = new PriorityQueue<Integer>();
                prioQueue.offer(maxLen);
                map.put(num, prioQueue);
            }
        }

        for (var entry : map.entrySet()) {
            if (entry.getValue().isEmpty()) {
                continue;
            }

            if (entry.getValue().peek() < 3) {
                return false;
            }
        }

        return true;
    }
}
