import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;

public class Main {
    public static void main(final String[] args) {
        final var s = new Solution();

        final var n1 = new int[] { 1, 1, 1, 2, 2, 3 };
        final var k1 = 2;

        final var n2 = new int[] { 1 };
        final var k2 = 1;

        // Expecting [1, 2]
        System.out.println(Arrays.toString(s.topKFrequent(n1, k1)));
        // Expecting [1]
        System.out.println(Arrays.toString(s.topKFrequent(n2, k2)));
    }
}

class Solution {
    public int[] topKFrequent(final int[] nums, final int k) {
        final var freqMap = new HashMap<Integer, Integer>();
        for (final var num : nums) {
            freqMap.put(num, freqMap.getOrDefault(num, 0) + 1);
        }

        final var minHeap = new PriorityQueue<Map.Entry<Integer, Integer>>((o1, o2) -> o1.getValue() - o2.getValue());
        for (final var entry : freqMap.entrySet()) {
            minHeap.offer(entry);
            if (minHeap.size() > k) {
                minHeap.poll();
            }
        }

        final var ret = new int[k];
        for (var i = 0; i < k; i++) {
            ret[i] = minHeap.poll().getKey();
        }

        return ret;
    }
}