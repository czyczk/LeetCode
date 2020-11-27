public class Main {
    public static void main(String[] args) {
        var nums1 = new int[] { 3, 6, 9, 1 };
        var nums2 = new int[] { 10 };
        var nums3 = new int[] { 2, 13, 7, 3 };

        var s = new Solution();
        s.radixSort(nums3);
        for (var num : nums3) {
            System.out.println(num);
        }
    }
}