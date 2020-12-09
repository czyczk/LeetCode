public class Main {
    public static void main(String[] args) {
        var m1 = 3;
        var n1 = 7;
        var m2 = 3;
        var n2 = 2;
        var m3 = 7;
        var n3 = 3;
        var m4 = 3;
        var n4 = 3;
        var m5 = 1;
        var n5 = 1;
        var m6 = 23;
        var n6 = 12;

        var s = new SolutionMath();
        // Expecting 28
        System.out.println(s.uniquePaths(m1, n1));
        // Expecting 3
        System.out.println(s.uniquePaths(m2, n2));
        // Expecting 28
        System.out.println(s.uniquePaths(m3, n3));
        // Expecting 6
        System.out.println(s.uniquePaths(m4, n4));
        // Expecting 1
        System.out.println(s.uniquePaths(m5, n5));
        // Expecting 193536720
        System.out.println(s.uniquePaths(m6, n6));
    }
}