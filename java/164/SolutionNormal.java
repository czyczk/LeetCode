import java.util.LinkedList;

public class SolutionNormal {
    public int maximumGap(int[] nums) {
        if (nums == null || nums.length < 2) {
            return 0;
        }

        int maxGap = 0;
        radixSort(nums);

        for (int i = 0; i < nums.length - 1; i++) {
            var gap = nums[i + 1] - nums[i];
            if (gap > maxGap) {
                maxGap = gap;
            }
        }

        return maxGap;
    }

    public void radixSort(int[] nums) {
        // Each bucket is a list. Space: 10n
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

        var exp = 1;
        for (var i = 0; i < maxStrLength; i++) {
            for (var num : nums) {
                buckets[num % (exp * 10) / exp].add(num);
            }

            var numsCursor = 0;

            for (var j = 0; j < buckets.length; j++) {
                while (buckets[j].size() != 0) {
                    nums[numsCursor++] = buckets[j].removeFirst();
                }
            }

            exp *= 10;
        }
    }
}
