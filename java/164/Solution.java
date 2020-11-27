import java.util.LinkedList;

public class Solution {
    public int maximumGap(int[] nums) {
        if (nums == null || nums.length < 2) {
            return 0;
        }

        int maxGap = 0;
        radixSort(nums);

        return 0;
    }

    public void radixSort(int[] nums) {
        LinkedList<Integer>[] buckets = new LinkedList[10];
        for (int i = 0; i < buckets.length; i++) {
            buckets[i] = new LinkedList<Integer>();
        }

        var maxStrLength = 0;
        for (var num : nums) {
            var numStrLen = ((Integer) num).toString().length();
            if (numStrLen > maxStrLength) {
                maxStrLength = numStrLen;
            }
        }

        for (var i = 0; i < maxStrLength; i++) {
            for (var num : nums) {
                buckets[num % (int) (Math.pow(10, i))].add(num);
            }

            var numsCursor = 0;

            for (var j = 0; j < buckets.length; j++) {
                while (buckets[j].size() != 0) {
                    nums[numsCursor++] = buckets[j].removeFirst();
                }
            }
        }
    }
}
