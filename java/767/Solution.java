import java.util.Comparator;
import java.util.PriorityQueue;

public class Solution {
    public String reorganizeString(String S) {
        var n = S.length();

        if (n < 2) {
            return S;
        }

        var buckets = new int[26];
        S.chars().forEach(c -> buckets[toIndex((char) c)]++);

        var maxCount = 0;
        for (var i = 0; i < 26; i++) {
            maxCount = Math.max(maxCount, buckets[i]);
        }

        if (maxCount > (n + 1) / 2) {
            return "";
        }

        char[] result = new char[n];
        int rCursor = 0;
        PriorityQueue<Character> maxHeap = new PriorityQueue<Character>(new Comparator<>() {
            public int compare(Character ch1, Character ch2) {
                return buckets[toIndex(ch2)] - buckets[toIndex(ch1)];
            }
        });

        for (char c = 'a'; c <= 'z'; c++) {
            if (buckets[toIndex(c)] > 0) {
                maxHeap.offer(c);
            }
        }

        while (maxHeap.size() > 1) {
            char c1 = maxHeap.poll();
            char c2 = maxHeap.poll();

            result[rCursor++] = c1;
            result[rCursor++] = c2;

            var idx1 = toIndex(c1);
            var idx2 = toIndex(c2);

            buckets[idx1]--;
            buckets[idx2]--;

            if (buckets[idx1] > 0) {
                maxHeap.offer(c1);
            }
            if (buckets[idx2] > 0) {
                maxHeap.offer(c2);
            }
        }

        if (maxHeap.size() > 0) {
            result[rCursor++] = maxHeap.poll();
        }

        return new String(result);
    }

    private int toIndex(char ch) {
        return ch - 'a';
    }
}
