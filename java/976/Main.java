public class Main {
    public static void main(String[] args) {
        var arr1 = new int[] { 2, 1, 2 };
        var arr2 = new int[] { 1, 2, 1 };
        var arr3 = new int[] { 3, 2, 3, 4 };
        var arr4 = new int[] { 3, 6, 2, 3 };
        var arr5 = new int[] { 3, 9, 2, 5, 2, 19 };
        var arr6 = new int[] { 3, 2, 3, 4 };

        var s = new Solution();

        // Expecting 5
        System.out.println(s.largestPerimeter(arr1));
        // Expecting 0
        System.out.println(s.largestPerimeter(arr2));
        // Expecting 10
        System.out.println(s.largestPerimeter(arr3));
        // Expecting 8
        System.out.println(s.largestPerimeter(arr4));
        // Expecting 7
        System.out.println(s.largestPerimeter(arr5));
        // Expecting 10
        System.out.println(s.largestPerimeter(arr6));
    }
}