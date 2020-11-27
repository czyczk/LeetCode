public class Solution {
    public String sortString(String s) {
        int[] buckets = new int[26];

        for (char c : s.toCharArray()) {
            buckets[toBucketIndex(c)]++;
        }

        var result = new StringBuilder();
        boolean isAllClear;
        var cursor = 0;
        while (true) {
            isAllClear = true;

            for (cursor = 0; cursor < 26; cursor++) {
                if (!consumeBuckets(buckets, cursor, result)) {
                    isAllClear = false;
                }
            }

            for (cursor = 25; cursor >= 0; cursor--) {
                if (!consumeBuckets(buckets, cursor, result)) {
                    isAllClear = false;
                }
            }

            if (isAllClear) {
                break;
            }
        }

        return result.toString();
    }

    private int toBucketIndex(char c) {
        return (int) c - (int) 'a';
    }

    private char toChar(int index) {
        return (char) (index + (int) 'a');
    }

    private boolean consumeBuckets(int[] buckets, int cursor, StringBuilder result) {
        if (buckets[cursor] != 0) {
            result.append(toChar(cursor));
            buckets[cursor]--;
            return false;
        }

        return true;
    }
}