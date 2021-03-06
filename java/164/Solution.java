public class Solution {
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
        var maxStrLength = 0;
        for (var num : nums) {
            var numStrLen = ((Integer) num).toString().length();
            if (numStrLen > maxStrLength) {
                maxStrLength = numStrLen;
            }
        }

        var buf = new int[nums.length];
        var exp = 1;
        for (var i = 0; i < maxStrLength; i++) {
            var buckets = new int[10];

            for (var num : nums) {
                buckets[(num / exp) % 10]++;
            }

            for (var j = 0; j < buckets.length - 1; j++) {
                buckets[j + 1] += buckets[j];
            }

            for (var j = nums.length - 1; j >= 0; j--) {
                var num = nums[j];
                var index = (num / exp) % 10;
                buf[buckets[index] - 1] = num;
                buckets[index]--;
            }

            exp *= 10;

            System.arraycopy(buf, 0, nums, 0, nums.length);
        }
    }
}
