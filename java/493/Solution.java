public class Solution {
    public int reversePairs(int[] nums) {
        var count = 0;

        for (var i = 0; i < nums.length - 1; i++) {
            for (var j = i + 1; j < nums.length; j++) {
                if (nums[i] > (long) nums[j] << 1) {
                    count++;
                }
            }
        }

        return count;
    }
}
