public class Main {
    public static void main(String[] args) {
        var nums1 = new int[] { 3, 6, 9, 1 };
        var nums2 = new int[] { 10 };
        var nums3 = new int[] { 2, 13, 7, 3 };
        var nums4 = new int[] { 1, 10000000 };

        var s = new Solution();
        // Expecting 3
        System.out.println(s.maximumGap(nums1));
        // Expecting 0
        System.out.println(s.maximumGap(nums2));
        // Expecting 6
        System.out.println(s.maximumGap(nums3));
        // Expecting 9999999
        System.out.println(s.maximumGap(nums4));
    }
}